macro_rules! execute_pseudo_indexed_range {
    () => {
        # [test] fn execute_pseudo_indexed_range () { let range = i128 :: MAX - 1024 .. i128 :: MAX ; let a = range . clone () . into_par_iter () ; assert_eq ! (a . opt_len () , Some (1024)) ; let b : Vec < i128 > = a . map (| i | i + 1) . collect () ; let c : Vec < i128 > = range . map (| i | i + 1) . collect () ; assert_eq ! (b , c) ; }
    };
}

execute_pseudo_indexed_range!()