// Generated macro for check_chunks_len (function)
macro_rules! Depcrate_iter_testcheck_chunks_len {
() => {
// Module: crate::iter::test
// Provides: {"check_chunks_len"}
// Dependencies: {}
# [test] fn check_chunks_len () { assert_eq ! (4 , (0 .. 8) . into_par_iter () . chunks (2) . len ()) ; assert_eq ! (3 , (0 .. 9) . into_par_iter () . chunks (3) . len ()) ; assert_eq ! (3 , (0 .. 8) . into_par_iter () . chunks (3) . len ()) ; assert_eq ! (1 , [1] . par_iter () . chunks (3) . len ()) ; assert_eq ! (0 , (0 .. 0) . into_par_iter () . chunks (3) . len ()) ; }
};
}
