macro_rules! execute_unindexed_range {
    () => {
        # [test] fn execute_unindexed_range () { let a = 0i64 .. 1024 ; let b : LinkedList < i64 > = a . into_par_iter () . map (| i | i + 1) . collect () ; let c : LinkedList < i64 > = (0 .. 1024) . map (| i | i + 1) . collect () ; assert_eq ! (b , c) ; }
    };
}

execute_unindexed_range!();