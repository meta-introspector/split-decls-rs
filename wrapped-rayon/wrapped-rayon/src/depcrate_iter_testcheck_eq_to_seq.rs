// Generated macro for check_eq_to_seq (function)
macro_rules! Depcrate_iter_testcheck_eq_to_seq {
() => {
// Module: crate::iter::test
// Provides: {"check_eq_to_seq"}
// Dependencies: {}
# [test] fn check_eq_to_seq () { let par_result = (0 .. 1024) . into_par_iter () . eq ((0 .. 1024) . into_par_iter ()) ; let seq_result = (0 .. 1024) . eq (0 .. 1024) ; assert_eq ! (par_result , seq_result) ; }
};
}
