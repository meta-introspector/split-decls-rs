// Generated macro for check_step_by_unaligned (function)
macro_rules! Depcrate_iter_testcheck_step_by_unaligned {
() => {
// Module: crate::iter::test
// Provides: {"check_step_by_unaligned"}
// Dependencies: {}
# [test] fn check_step_by_unaligned () { let a : Vec < i32 > = (0 .. 1029) . step_by (10) . collect () ; let b : Vec < i32 > = (0 .. 1029) . into_par_iter () . step_by (10) . collect () ; assert_eq ! (a , b) }
};
}
