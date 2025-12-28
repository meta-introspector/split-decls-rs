macro_rules! check_gt_to_seq {
    () => {
        # [test] fn check_gt_to_seq () { let par_result = (1 .. 1024) . into_par_iter () . gt ((0 .. 1024) . into_par_iter ()) ; let seq_result = (1 .. 1024) . gt (0 .. 1024) ; assert_eq ! (par_result , seq_result) ; }
    };
}

check_gt_to_seq!();