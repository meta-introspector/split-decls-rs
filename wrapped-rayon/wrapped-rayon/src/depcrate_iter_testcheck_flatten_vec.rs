// Generated macro for check_flatten_vec (function)
macro_rules! Depcrate_iter_testcheck_flatten_vec {
() => {
// Module: crate::iter::test
// Provides: {"check_flatten_vec"}
// Dependencies: {}
# [test] fn check_flatten_vec () { let a : Vec < i32 > = (0 .. 1024) . collect () ; let b : Vec < Vec < i32 > > = vec ! [a . clone () , a . clone () , a . clone () , a . clone ()] ; let c : Vec < i32 > = b . par_iter () . flatten () . cloned () . collect () ; let mut d = a . clone () ; d . extend (& a) ; d . extend (& a) ; d . extend (& a) ; assert_eq ! (d , c) ; }
};
}
