// Generated macro for check_enumerate (function)
macro_rules! Depcrate_iter_testcheck_enumerate {
() => {
// Module: crate::iter::test
// Provides: {"check_enumerate"}
// Dependencies: {}
# [test] fn check_enumerate () { let a : Vec < usize > = (0 .. 1024) . rev () . collect () ; let mut b = vec ! [] ; a . par_iter () . enumerate () . map (| (i , & x) | i + x) . collect_into_vec (& mut b) ; assert ! (b . iter () . all (|& x | x == a . len () - 1)) ; }
};
}
