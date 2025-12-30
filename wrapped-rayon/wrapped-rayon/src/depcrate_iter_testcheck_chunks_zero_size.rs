// Generated macro for check_chunks_zero_size (function)
macro_rules! Depcrate_iter_testcheck_chunks_zero_size {
() => {
// Module: crate::iter::test
// Provides: {"check_chunks_zero_size"}
// Dependencies: {}
# [test] # [should_panic (expected = "chunk_size must not be zero")] fn check_chunks_zero_size () { let _ : Vec < Vec < i32 > > = vec ! [1 , 2 , 3] . into_par_iter () . chunks (0) . collect () ; }
};
}
