// Generated macro for test_replace (function)
macro_rules! Depcrate_tests_mutable_datatest_replace {
() => {
// Module: crate::tests::mutable_data
// Provides: {"test_replace"}
// Dependencies: {}
# [test] # [cfg (feature = "NSRange")] fn test_replace () { let data = NSMutableData :: with_bytes (& [7 , 16]) ; data . replace_range (0 .. 0 , & [3]) ; assert_eq ! (data . to_vec () , [3 , 7 , 16]) ; data . replace_range (1 .. 2 , & [52 , 13]) ; assert_eq ! (data . to_vec () , [3 , 52 , 13 , 16]) ; data . replace_range (2 .. 4 , & [6]) ; assert_eq ! (data . to_vec () , [3 , 52 , 6]) ; data . set_bytes (& [8 , 17]) ; assert_eq ! (data . to_vec () , [8 , 17]) ; }
};
}
