macro_rules! check_partial_cmp_late_nan_to_seq {
    () => {
        # [test] fn check_partial_cmp_late_nan_to_seq () { let a = vec ! [0.0 , f64 :: NAN] ; let b = vec ! [1.0 , 1.0] ; let par_result = a . par_iter () . partial_cmp (b . par_iter ()) ; let seq_result = a . iter () . partial_cmp (b . iter ()) ; assert_eq ! (par_result , seq_result) ; }
    };
}

check_partial_cmp_late_nan_to_seq!()