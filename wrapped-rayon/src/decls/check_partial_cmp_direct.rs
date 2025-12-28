macro_rules! check_partial_cmp_direct {
    () => {
        # [test] fn check_partial_cmp_direct () { let a = (0 .. 1024) . into_par_iter () ; let b = (0 .. 1024) . into_par_iter () ; let result = a . partial_cmp (b) ; assert ! (result == Some (:: std :: cmp :: Ordering :: Equal)) ; }
    };
}

check_partial_cmp_direct!()