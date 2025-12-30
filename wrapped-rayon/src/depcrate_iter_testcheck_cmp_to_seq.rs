// Generated macro for check_cmp_to_seq (function)
macro_rules! Depcrate_iter_testcheck_cmp_to_seq {
() => {
// Module: crate::iter::test
// Provides: {"check_cmp_to_seq"}
// Dependencies: {}
# [test] fn check_cmp_to_seq () { assert_eq ! ((0 .. 1024) . into_par_iter () . cmp (0 .. 1024) , (0 .. 1024) . cmp (0 .. 1024)) ; }
};
}
