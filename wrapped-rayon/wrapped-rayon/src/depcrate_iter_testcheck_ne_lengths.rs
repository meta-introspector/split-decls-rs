// Generated macro for check_ne_lengths (function)
macro_rules! Depcrate_iter_testcheck_ne_lengths {
() => {
// Module: crate::iter::test
// Provides: {"check_ne_lengths"}
// Dependencies: {}
# [test] fn check_ne_lengths () { let a = vec ! [0 ; 1024] ; let b = vec ! [0 ; 1025] ; assert_eq ! (a . par_iter () . eq (& b) , a . iter () . eq (& b)) ; assert_eq ! (a . par_iter () . ne (& b) , a . iter () . ne (& b)) ; }
};
}
