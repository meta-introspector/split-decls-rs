// Generated macro for check_flatten_vec_empty (function)
macro_rules! Depcrate_iter_testcheck_flatten_vec_empty {
() => {
// Module: crate::iter::test
// Provides: {"check_flatten_vec_empty"}
// Dependencies: {}
# [test] fn check_flatten_vec_empty () { let a : Vec < Vec < i32 > > = vec ! [vec ! []] ; let b : Vec < i32 > = a . par_iter () . flatten () . cloned () . collect () ; assert_eq ! (vec ! [] as Vec < i32 >, b) ; }
};
}
