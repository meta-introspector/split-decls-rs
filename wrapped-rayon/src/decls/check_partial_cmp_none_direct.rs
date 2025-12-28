macro_rules! check_partial_cmp_none_direct {
    () => {
        # [test] fn check_partial_cmp_none_direct () { let a = vec ! [f64 :: NAN , 0.0] ; let b = vec ! [0.0 , 1.0] ; let result = a . par_iter () . partial_cmp (b . par_iter ()) ; assert ! (result . is_none ()) ; }
    };
}

check_partial_cmp_none_direct!()