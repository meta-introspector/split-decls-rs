macro_rules! check_cmp_direct {
    () => {
        # [test] fn check_cmp_direct () { let a = (0 .. 1024) . into_par_iter () ; let b = (0 .. 1024) . into_par_iter () ; let result = a . cmp (b) ; assert ! (result == :: std :: cmp :: Ordering :: Equal) ; }
    };
}

check_cmp_direct!()