// Generated macro for test_with_capacity (function)
macro_rules! Depcrate_tests_mutable_datatest_with_capacity {
() => {
// Module: crate::tests::mutable_data
// Provides: {"test_with_capacity"}
// Dependencies: {}
# [test] fn test_with_capacity () { let data = NSMutableData :: dataWithCapacity (5) . unwrap () ; assert_eq ! (data . to_vec () , & []) ; data . extend_from_slice (& [1 , 2 , 3 , 4 , 5]) ; assert_eq ! (data . to_vec () , & [1 , 2 , 3 , 4 , 5]) ; data . extend_from_slice (& [6 , 7]) ; assert_eq ! (data . to_vec () , & [1 , 2 , 3 , 4 , 5 , 6 , 7]) ; }
};
}
