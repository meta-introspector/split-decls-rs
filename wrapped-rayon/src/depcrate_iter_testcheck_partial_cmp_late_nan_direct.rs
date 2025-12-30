// Generated macro for check_partial_cmp_late_nan_direct (function)
macro_rules! Depcrate_iter_testcheck_partial_cmp_late_nan_direct {
() => {
// Module: crate::iter::test
// Provides: {"check_partial_cmp_late_nan_direct"}
// Dependencies: {}
# [test] fn check_partial_cmp_late_nan_direct () { let a = vec ! [0.0 , f64 :: NAN] ; let b = vec ! [1.0 , 1.0] ; let result = a . par_iter () . partial_cmp (b . par_iter ()) ; assert ! (result == Some (:: std :: cmp :: Ordering :: Less)) ; }
};
}
