// Generated macro for check_move (function)
macro_rules! Depcrate_iter_testcheck_move {
() => {
// Module: crate::iter::test
// Provides: {"check_move"}
// Dependencies: {}
# [test] fn check_move () { let a = vec ! [vec ! [1 , 2 , 3]] ; let ptr = a [0] . as_ptr () ; let mut b = vec ! [] ; a . into_par_iter () . collect_into_vec (& mut b) ; assert_eq ! (ptr , b [0] . as_ptr ()) ; }
};
}
