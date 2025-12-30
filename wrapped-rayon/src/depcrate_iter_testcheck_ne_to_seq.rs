// Generated macro for check_ne_to_seq (function)
macro_rules! Depcrate_iter_testcheck_ne_to_seq {
() => {
// Module: crate::iter::test
// Provides: {"check_ne_to_seq"}
// Dependencies: {}
# [test] fn check_ne_to_seq () { let par_result = (0 .. 1024) . into_par_iter () . ne ((1 .. 1025) . into_par_iter ()) ; let seq_result = (0 .. 1024) . ne (1 .. 1025) ; assert_eq ! (par_result , seq_result) ; }
};
}
