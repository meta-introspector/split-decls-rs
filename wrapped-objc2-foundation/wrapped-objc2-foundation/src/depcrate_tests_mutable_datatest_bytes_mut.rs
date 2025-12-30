// Generated macro for test_bytes_mut (function)
macro_rules! Depcrate_tests_mutable_datatest_bytes_mut {
() => {
// Module: crate::tests::mutable_data
// Provides: {"test_bytes_mut"}
// Dependencies: {}
# [test] fn test_bytes_mut () { let data = NSMutableData :: with_bytes (& [7 , 16]) ; unsafe { data . as_mut_bytes_unchecked () [0] = 3 } ; assert_eq ! (data . to_vec () , [3 , 16]) ; }
};
}
