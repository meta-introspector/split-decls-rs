// Generated macro for test_set_len (function)
macro_rules! Depcrate_tests_mutable_datatest_set_len {
() => {
// Module: crate::tests::mutable_data
// Provides: {"test_set_len"}
// Dependencies: {}
# [test] fn test_set_len () { let data = NSMutableData :: with_bytes (& [7 , 16]) ; data . setLength (4) ; assert_eq ! (data . len () , 4) ; assert_eq ! (data . to_vec () , [7 , 16 , 0 , 0]) ; data . setLength (1) ; assert_eq ! (data . len () , 1) ; assert_eq ! (data . to_vec () , [7]) ; }
};
}
