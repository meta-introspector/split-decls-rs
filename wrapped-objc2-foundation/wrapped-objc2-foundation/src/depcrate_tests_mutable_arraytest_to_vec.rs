// Generated macro for test_to_vec (function)
macro_rules! Depcrate_tests_mutable_arraytest_to_vec {
() => {
// Module: crate::tests::mutable_array
// Provides: {"test_to_vec"}
// Dependencies: {}
# [test] # [cfg (feature = "NSString")] # [cfg_attr (feature = "gnustep-1-7" , ignore = "thread safety issues regarding initialization")] fn test_to_vec () { let array = NSMutableArray :: from_retained_slice (& [crate :: NSString :: new ()]) ; let vec = array . to_vec () ; assert_eq ! (vec . len () , 1) ; }
};
}
