macro_rules! check_cmp_lt_direct {
    () => {
        # [test] fn check_cmp_lt_direct () { let a = (0 .. 1024) . into_par_iter () ; let b = (1 .. 1024) . into_par_iter () ; let result = a . cmp (b) ; assert ! (result == :: std :: cmp :: Ordering :: Less) ; }
    };
}

check_cmp_lt_direct!();