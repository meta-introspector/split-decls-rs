// Generated macro for test_append (function)
macro_rules! Depcrate_tests_mutable_datatest_append {
() => {
// Module: crate::tests::mutable_data
// Provides: {"test_append"}
// Dependencies: {}
# [test] fn test_append () { let data = NSMutableData :: with_bytes (& [7 , 16]) ; data . extend_from_slice (& [3 , 52]) ; assert_eq ! (data . len () , 4) ; assert_eq ! (data . to_vec () , [7 , 16 , 3 , 52]) ; }
};
}
