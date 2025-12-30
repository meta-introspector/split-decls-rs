// Generated macro for check_partial_cmp_gt_to_seq (function)
macro_rules! Depcrate_iter_testcheck_partial_cmp_gt_to_seq {
() => {
// Module: crate::iter::test
// Provides: {"check_partial_cmp_gt_to_seq"}
// Dependencies: {}
# [test] fn check_partial_cmp_gt_to_seq () { let par_result = (1 .. 1024) . into_par_iter () . partial_cmp (0 .. 1024) ; let seq_result = (1 .. 1024) . partial_cmp (0 .. 1024) ; assert_eq ! (par_result , seq_result) ; }
};
}
