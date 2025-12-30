// Generated macro for test_from_vec (function)
macro_rules! Depcrate_tests_datatest_from_vec {
() => {
// Module: crate::tests::data
// Provides: {"test_from_vec"}
// Dependencies: {}
# [cfg (feature = "block2")] # [test] fn test_from_vec () { let bytes = alloc :: vec ! [3 , 7 , 16] ; let bytes_ptr = bytes . as_ptr () ; let data = NSData :: from_vec (bytes) ; assert_eq ! (unsafe { data . as_bytes_unchecked () } . as_ptr () , bytes_ptr) ; }
};
}
