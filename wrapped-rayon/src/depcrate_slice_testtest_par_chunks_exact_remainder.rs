// Generated macro for test_par_chunks_exact_remainder (function)
macro_rules! Depcrate_slice_testtest_par_chunks_exact_remainder {
() => {
// Module: crate::slice::test
// Provides: {"test_par_chunks_exact_remainder"}
// Dependencies: {}
# [test] fn test_par_chunks_exact_remainder () { let v : & [i32] = & [0 , 1 , 2 , 3 , 4] ; let c = v . par_chunks_exact (2) ; assert_eq ! (c . remainder () , & [4]) ; assert_eq ! (c . len () , 2) ; }
};
}
