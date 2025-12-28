macro_rules! check_le_less_to_seq {
    () => {
        # [test] fn check_le_less_to_seq () { let par_result = (0 .. 1024) . into_par_iter () . le ((1 .. 1024) . into_par_iter ()) ; let seq_result = (0 .. 1024) . le (1 .. 1024) ; assert_eq ! (par_result , seq_result) ; }
    };
}

check_le_less_to_seq!();