macro_rules! check_partial_cmp_gt_direct {
    () => {
        # [test] fn check_partial_cmp_gt_direct () { let a = (1 .. 1024) . into_par_iter () ; let b = (0 .. 1024) . into_par_iter () ; let result = a . partial_cmp (b) ; assert ! (result == Some (:: std :: cmp :: Ordering :: Greater)) ; }
    };
}

check_partial_cmp_gt_direct!();