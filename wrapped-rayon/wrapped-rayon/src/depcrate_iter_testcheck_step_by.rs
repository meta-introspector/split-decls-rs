// Generated macro for check_step_by (function)
macro_rules! Depcrate_iter_testcheck_step_by {
() => {
// Module: crate::iter::test
// Provides: {"check_step_by"}
// Dependencies: {}
# [test] fn check_step_by () { let a : Vec < i32 > = (0 .. 1024) . step_by (2) . collect () ; let b : Vec < i32 > = (0 .. 1024) . into_par_iter () . step_by (2) . collect () ; assert_eq ! (a , b) ; }
};
}
