// Generated macro for check_ne_direct (function)
macro_rules! Depcrate_iter_testcheck_ne_direct {
() => {
// Module: crate::iter::test
// Provides: {"check_ne_direct"}
// Dependencies: {}
# [test] fn check_ne_direct () { let a = (0 .. 1024) . into_par_iter () ; let b = (1 .. 1024) . into_par_iter () ; let result = a . ne (b) ; assert ! (result) ; }
};
}
