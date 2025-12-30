// Generated macro for check_cmp_lt_direct (function)
macro_rules! Depcrate_iter_testcheck_cmp_lt_direct {
() => {
// Module: crate::iter::test
// Provides: {"check_cmp_lt_direct"}
// Dependencies: {}
# [test] fn check_cmp_lt_direct () { let a = (0 .. 1024) . into_par_iter () ; let b = (1 .. 1024) . into_par_iter () ; let result = a . cmp (b) ; assert ! (result == :: std :: cmp :: Ordering :: Less) ; }
};
}
