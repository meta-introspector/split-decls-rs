macro_rules! execute_range {
    () => {
        # [test] fn execute_range () { let a = 0i32 .. 1024 ; let mut b = vec ! [] ; a . into_par_iter () . map (| i | i + 1) . collect_into_vec (& mut b) ; let c : Vec < i32 > = (0 .. 1024) . map (| i | i + 1) . collect () ; assert_eq ! (b , c) ; }
    };
}

execute_range!();