macro_rules! test_par_sort_stability {
    () => {
        # [test] fn test_par_sort_stability () { for len in (2 .. 25) . chain (500 .. 510) . chain (50_000 .. 50_010) { for _ in 0 .. 10 { let mut counts = [0 ; 10] ; let mut rng = rng () ; let mut v : Vec < _ > = (0 .. len) . map (| _ | { let n : usize = rng . random_range (0 .. 10) ; counts [n] += 1 ; (n , counts [n]) }) . collect () ; v . par_sort_by (| & (a , _) , & (b , _) | a . cmp (& b)) ; assert ! (v . windows (2) . all (| w | w [0] <= w [1])) ; } } }
    };
}

test_par_sort_stability!()