// Generated macro for check_zip (function)
macro_rules! Depcrate_iter_testcheck_zip {
() => {
// Module: crate::iter::test
// Provides: {"check_zip"}
// Dependencies: {}
# [test] fn check_zip () { let mut a : Vec < usize > = (0 .. 1024) . rev () . collect () ; let b : Vec < usize > = (0 .. 1024) . collect () ; a . par_iter_mut () . zip (& b [..]) . for_each (| (a , & b) | * a += b) ; assert ! (a . iter () . all (|& x | x == a . len () - 1)) ; }
};
}
