// Generated macro for check_lt_direct (function)
macro_rules! Depcrate_iter_testcheck_lt_direct {
() => {
// Module: crate::iter::test
// Provides: {"check_lt_direct"}
// Dependencies: {}
# [test] fn check_lt_direct () { assert ! ((0 .. 1024) . into_par_iter () . lt (1 .. 1024)) ; assert ! (! (1 .. 1024) . into_par_iter () . lt (0 .. 1024)) ; }
};
}
