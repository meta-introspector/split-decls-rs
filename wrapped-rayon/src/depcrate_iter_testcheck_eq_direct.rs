// Generated macro for check_eq_direct (function)
macro_rules! Depcrate_iter_testcheck_eq_direct {
() => {
// Module: crate::iter::test
// Provides: {"check_eq_direct"}
// Dependencies: {}
# [test] fn check_eq_direct () { let a = (0 .. 1024) . into_par_iter () ; let b = (0 .. 1024) . into_par_iter () ; let result = a . eq (b) ; assert ! (result) ; }
};
}
