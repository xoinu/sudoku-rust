type Board = [[u8; 9]; 9];

fn num_empty_cells(mtx: &Board) -> i32 {
    let mut n = 0;

    for i in 0..9 {
        let row = &mtx[i];
        for j in 0..9 {
            if row[j] == 0 {
                n += 1;
            }
        }
    }

    return n;
}

fn print_mtx(mtx: &Board) {
    for i in 0..9 {
        if i % 3 == 0 {
            println!("+-------+-------+-------+");
        }
        let row = &mtx[i];
        println!(
            "| {} {} {} | {} {} {} | {} {} {} |",
            row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7], row[8]
        );
    }
    println!("+-------+-------+-------+");
}

fn calc_candidate_at(mtx: &Board, row: usize, col: usize) -> ([u8; 9], usize) {
    let mut cand: [u8; 9] = [0; 9];
    let mut num = 0;

    if mtx[row][col] != 0 {
        return (cand, num);
    }

    let mut flags = [false; 9];
    let r = &mtx[row];

    for i in 0..9 {
        let v = r[i] as usize;
        if v != 0 {
            flags[v - 1] = true;
        }
        let v = mtx[i][col] as usize;
        if v != 0 {
            flags[v - 1] = true;
        }
    }

    let row_block = ((row / 3) * 3) as usize;
    let col_block = ((col / 3) * 3) as usize;

    for i in 0..3 {
        for j in 0..3 {
            let v = mtx[i + row_block][j + col_block] as usize;
            if v != 0 {
                flags[v - 1] = true;
            }
        }
    }

    for i in 0..9 {
        if !flags[i] {
            cand[num as usize] = (i + 1) as u8;
            num += 1;
        }
    }

    return (cand, num);
}

fn calc_candidates(mtx: &Board) -> ([u8; 9], usize, usize, usize) {
    let mut min_cand = [0 as u8; 9];
    let mut num_min_cand = 10;
    let mut min_cand_row = 0;
    let mut min_cand_col = 0;

    for i in 0..9 {
        for j in 0..9 {
            let (cand, num) = calc_candidate_at(mtx, i, j);
            if num == 0 {
                continue;
            }
            print!("[{},{}] ...", i + 1, j + 1);
            for k in 0..num {
                print!(" {}", cand[k]);
            }
            println!("");

            if num == 1 {
                return (cand, 1, i, j);
            }

            if num <= num_min_cand {
                num_min_cand = num;
                min_cand_row = i;
                min_cand_col = j;
                min_cand = cand;
            }
        }
    }

    return (min_cand, num_min_cand, min_cand_row, min_cand_col);
}

fn solve_one(mtx: &mut Board, n: i32) -> bool {
    if n == 0 {
        return true;
    }

    let (cand, num_cand, i, j) = calc_candidates(mtx);
    println!("n={}, ({}, {}): num_cand={}, cand={:?}", n, i, j, num_cand, cand);

    if num_cand == 10 {
        println!("No more candidate!");
        print_mtx(mtx);
        return false;
    }

    for k in 0..num_cand {
        mtx[i][j] = cand[k];
        println!("n={}: try ({},{}) => {}...", n, i, j, cand[k]);
        print_mtx(mtx);
        if solve_one(mtx, n - 1) {
            return true;
        }
        println!("n={}: ({},{})={} was a bad guess...", n, i, j, cand[k]);
        mtx[i][j] = 0;
    }

    return false;
}

fn main() {
    let mut mtx = [
        [0, 0, 0, 0, 8, 0, 0, 6, 0],
        [0, 0, 0, 1, 6, 0, 5, 0, 8],
        [8, 9, 0, 0, 4, 3, 0, 0, 2],
        [0, 5, 9, 0, 0, 0, 0, 0, 6],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [4, 0, 0, 0, 0, 0, 8, 5, 0],
        [2, 0, 0, 9, 5, 0, 0, 3, 7],
        [6, 0, 5, 0, 2, 1, 0, 0, 0],
        [0, 4, 0, 0, 3, 0, 0, 0, 0],
    ];
    // let mut mtx = [
    //     [0, 0, 2, 6, 4, 0, 0, 0, 8],
    //     [3, 0, 0, 0, 0, 7, 0, 4, 0],
    //     [0, 0, 0, 0, 0, 0, 0, 5, 0],
    //     [5, 1, 0, 4, 0, 0, 3, 0, 0],
    //     [0, 0, 7, 0, 0, 0, 4, 0, 0],
    //     [0, 0, 9, 0, 0, 1, 0, 7, 2],
    //     [0, 7, 0, 0, 0, 0, 0, 0, 0],
    //     [0, 9, 0, 7, 0, 0, 0, 0, 4],
    //     [2, 0, 0, 0, 3, 6, 1, 0, 0],
    // ];
    print_mtx(&mtx);
    let n = num_empty_cells(&mtx);

    if solve_one(&mut mtx, n) {
        println!("Solved.");
    } else {
        println!("Gee!!!");
    }
}
