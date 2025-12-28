macro_rules! check_partial_cmp_none_to_seq {
    () => {
        # [test] fn check_partial_cmp_none_to_seq () { let a = vec ! [f64 :: NAN , 0.0] ; let b = vec ! [0.0 , 1.0] ; let par_result = a . par_iter () . partial_cmp (b . par_iter ()) ; let seq_result = a . iter () . partial_cmp (b . iter ()) ; assert_eq ! (par_result , seq_result) ; }
    };
}

check_partial_cmp_none_to_seq!();