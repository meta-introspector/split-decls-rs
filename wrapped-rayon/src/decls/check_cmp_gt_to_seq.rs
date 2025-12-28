macro_rules! check_cmp_gt_to_seq {
    () => {
        # [test] fn check_cmp_gt_to_seq () { assert_eq ! ((1 .. 1024) . into_par_iter () . cmp (0 .. 1024) , (1 .. 1024) . cmp (0 .. 1024)) }
    };
}

check_cmp_gt_to_seq!()