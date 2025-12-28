macro_rules! check_partial_cmp_late_nan_direct {
    () => {
        # [test] fn check_partial_cmp_late_nan_direct () { let a = vec ! [0.0 , f64 :: NAN] ; let b = vec ! [1.0 , 1.0] ; let result = a . par_iter () . partial_cmp (b . par_iter ()) ; assert ! (result == Some (:: std :: cmp :: Ordering :: Less)) ; }
    };
}

check_partial_cmp_late_nan_direct!()