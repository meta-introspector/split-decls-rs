macro_rules! check_ge_equal_to_seq {
    () => {
        # [test] fn check_ge_equal_to_seq () { let par_result = (0 .. 1024) . into_par_iter () . ge ((0 .. 1024) . into_par_iter ()) ; let seq_result = (0 .. 1024) . ge (0 .. 1024) ; assert_eq ! (par_result , seq_result) ; }
    };
}

check_ge_equal_to_seq!();