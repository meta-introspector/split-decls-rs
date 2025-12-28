macro_rules! check_partial_cmp_lt_to_seq {
    () => {
        # [test] fn check_partial_cmp_lt_to_seq () { let par_result = (0 .. 1024) . into_par_iter () . partial_cmp (1 .. 1024) ; let seq_result = (0 .. 1024) . partial_cmp (1 .. 1024) ; assert_eq ! (par_result , seq_result) ; }
    };
}

check_partial_cmp_lt_to_seq!();