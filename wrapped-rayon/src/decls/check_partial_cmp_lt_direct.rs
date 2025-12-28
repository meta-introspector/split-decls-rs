macro_rules! check_partial_cmp_lt_direct {
    () => {
        # [test] fn check_partial_cmp_lt_direct () { let a = (0 .. 1024) . into_par_iter () ; let b = (1 .. 1024) . into_par_iter () ; let result = a . partial_cmp (b) ; assert ! (result == Some (:: std :: cmp :: Ordering :: Less)) ; }
    };
}

check_partial_cmp_lt_direct!();