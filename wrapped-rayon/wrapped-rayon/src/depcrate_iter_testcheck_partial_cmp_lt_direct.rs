// Generated macro for check_partial_cmp_lt_direct (function)
macro_rules! Depcrate_iter_testcheck_partial_cmp_lt_direct {
() => {
// Module: crate::iter::test
// Provides: {"check_partial_cmp_lt_direct"}
// Dependencies: {}
# [test] fn check_partial_cmp_lt_direct () { let a = (0 .. 1024) . into_par_iter () ; let b = (1 .. 1024) . into_par_iter () ; let result = a . partial_cmp (b) ; assert ! (result == Some (:: std :: cmp :: Ordering :: Less)) ; }
};
}
