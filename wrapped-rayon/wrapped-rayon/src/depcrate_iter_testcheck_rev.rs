// Generated macro for check_rev (function)
macro_rules! Depcrate_iter_testcheck_rev {
() => {
// Module: crate::iter::test
// Provides: {"check_rev"}
// Dependencies: {}
# [test] fn check_rev () { let a : Vec < usize > = (0 .. 1024) . rev () . collect () ; let b : Vec < usize > = (0 .. 1024) . collect () ; assert ! (a . par_iter () . rev () . zip (b) . all (| (& a , b) | a == b)) ; }
};
}
