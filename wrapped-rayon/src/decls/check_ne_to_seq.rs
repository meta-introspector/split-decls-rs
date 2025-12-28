macro_rules! check_ne_to_seq {
    () => {
        # [test] fn check_ne_to_seq () { let par_result = (0 .. 1024) . into_par_iter () . ne ((1 .. 1025) . into_par_iter ()) ; let seq_result = (0 .. 1024) . ne (1 .. 1025) ; assert_eq ! (par_result , seq_result) ; }
    };
}

check_ne_to_seq!()