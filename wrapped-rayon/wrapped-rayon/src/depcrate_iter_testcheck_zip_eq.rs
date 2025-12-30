// Generated macro for check_zip_eq (function)
macro_rules! Depcrate_iter_testcheck_zip_eq {
() => {
// Module: crate::iter::test
// Provides: {"check_zip_eq"}
// Dependencies: {}
# [test] fn check_zip_eq () { let mut a : Vec < usize > = (0 .. 1024) . rev () . collect () ; let b : Vec < usize > = (0 .. 1024) . collect () ; a . par_iter_mut () . zip_eq (& b [..]) . for_each (| (a , & b) | * a += b) ; assert ! (a . iter () . all (|& x | x == a . len () - 1)) ; }
};
}
