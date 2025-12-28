macro_rules! check_cmp_gt_direct {
    () => {
        # [test] fn check_cmp_gt_direct () { let a = (1 .. 1024) . into_par_iter () ; let b = (0 .. 1024) . into_par_iter () ; let result = a . cmp (b) ; assert ! (result == :: std :: cmp :: Ordering :: Greater) ; }
    };
}

check_cmp_gt_direct!()