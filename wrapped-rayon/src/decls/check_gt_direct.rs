macro_rules! check_gt_direct {
    () => {
        # [test] fn check_gt_direct () { assert ! ((1 .. 1024) . into_par_iter () . gt ((0 .. 1024) . into_par_iter ())) ; }
    };
}

check_gt_direct!();