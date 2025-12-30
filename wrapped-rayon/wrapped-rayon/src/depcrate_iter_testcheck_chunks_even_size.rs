// Generated macro for check_chunks_even_size (function)
macro_rules! Depcrate_iter_testcheck_chunks_even_size {
() => {
// Module: crate::iter::test
// Provides: {"check_chunks_even_size"}
// Dependencies: {}
# [test] fn check_chunks_even_size () { assert_eq ! (vec ! [vec ! [1 , 2 , 3] , vec ! [4 , 5 , 6] , vec ! [7 , 8 , 9]] , (1 .. 10) . into_par_iter () . chunks (3) . collect ::< Vec < Vec < i32 >>> ()) ; }
};
}
