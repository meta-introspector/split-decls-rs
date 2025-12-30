// Generated macro for check_step_by_rev (function)
macro_rules! Depcrate_iter_testcheck_step_by_rev {
() => {
// Module: crate::iter::test
// Provides: {"check_step_by_rev"}
// Dependencies: {}
# [test] fn check_step_by_rev () { let a : Vec < i32 > = (0 .. 1024) . step_by (2) . rev () . collect () ; let b : Vec < i32 > = (0 .. 1024) . into_par_iter () . step_by (2) . rev () . collect () ; assert_eq ! (a , b) ; }
};
}
