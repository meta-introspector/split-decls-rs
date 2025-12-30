// Generated macro for check_zip_into_mut_par_iter (function)
macro_rules! Depcrate_iter_testcheck_zip_into_mut_par_iter {
() => {
// Module: crate::iter::test
// Provides: {"check_zip_into_mut_par_iter"}
// Dependencies: {}
# [test] fn check_zip_into_mut_par_iter () { let a : Vec < usize > = (0 .. 1024) . rev () . collect () ; let mut b : Vec < usize > = (0 .. 1024) . collect () ; a . par_iter () . zip (& mut b) . for_each (| (& a , b) | * b += a) ; assert ! (b . iter () . all (|& x | x == b . len () - 1)) ; }
};
}
