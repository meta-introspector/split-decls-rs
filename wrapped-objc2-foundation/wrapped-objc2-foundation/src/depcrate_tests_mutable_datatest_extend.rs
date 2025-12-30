// Generated macro for test_extend (function)
macro_rules! Depcrate_tests_mutable_datatest_extend {
() => {
// Module: crate::tests::mutable_data
// Provides: {"test_extend"}
// Dependencies: {}
# [test] fn test_extend () { let mut data = NSMutableData :: with_bytes (& [1 , 2]) ; data . extend (3 ..= 5) ; assert_eq ! (data . to_vec () , & [1 , 2 , 3 , 4 , 5]) ; (& data) . extend (& * NSData :: with_bytes (& [6 , 7])) ; assert_eq ! (data . to_vec () , & [1 , 2 , 3 , 4 , 5 , 6 , 7]) ; }
};
}
