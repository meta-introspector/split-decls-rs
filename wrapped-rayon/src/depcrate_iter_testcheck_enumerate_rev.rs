// Generated macro for check_enumerate_rev (function)
macro_rules! Depcrate_iter_testcheck_enumerate_rev {
() => {
// Module: crate::iter::test
// Provides: {"check_enumerate_rev"}
// Dependencies: {}
# [test] fn check_enumerate_rev () { let a : Vec < usize > = (0 .. 1024) . rev () . collect () ; let mut b = vec ! [] ; a . par_iter () . enumerate () . rev () . map (| (i , & x) | i + x) . collect_into_vec (& mut b) ; assert ! (b . iter () . all (|& x | x == a . len () - 1)) ; }
};
}
