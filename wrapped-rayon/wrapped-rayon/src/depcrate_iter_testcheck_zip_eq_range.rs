// Generated macro for check_zip_eq_range (function)
macro_rules! Depcrate_iter_testcheck_zip_eq_range {
() => {
// Module: crate::iter::test
// Provides: {"check_zip_eq_range"}
// Dependencies: {}
# [test] fn check_zip_eq_range () { let mut a : Vec < usize > = (0 .. 1024) . rev () . collect () ; a . par_iter_mut () . zip_eq (0usize .. 1024) . for_each (| (a , b) | * a += b) ; assert ! (a . iter () . all (|& x | x == a . len () - 1)) ; }
};
}
