// Generated macro for check_cmp_lengths (function)
macro_rules! Depcrate_iter_testcheck_cmp_lengths {
() => {
// Module: crate::iter::test
// Provides: {"check_cmp_lengths"}
// Dependencies: {}
# [test] fn check_cmp_lengths () { let a = vec ! [0 ; 1024] ; let b = vec ! [0 ; 1025] ; assert_eq ! (a . par_iter () . cmp (& b) , a . iter () . cmp (& b)) ; assert_eq ! (a . par_iter () . partial_cmp (& b) , a . iter () . partial_cmp (& b)) ; }
};
}
