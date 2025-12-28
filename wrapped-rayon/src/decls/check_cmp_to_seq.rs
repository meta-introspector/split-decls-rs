macro_rules! check_cmp_to_seq {
    () => {
        # [test] fn check_cmp_to_seq () { assert_eq ! ((0 .. 1024) . into_par_iter () . cmp (0 .. 1024) , (0 .. 1024) . cmp (0 .. 1024)) ; }
    };
}

check_cmp_to_seq!()