// Generated macro for test_containing_another_array (function)
macro_rules! Depcrate_tests_mutable_arraytest_containing_another_array {
() => {
// Module: crate::tests::mutable_array
// Provides: {"test_containing_another_array"}
// Dependencies: {}
# [test] # [cfg_attr (feature = "gnustep-1-7" , ignore = "thread safety issues regarding initialization")] fn test_containing_another_array () { let array = NSMutableArray :: from_retained_slice (& [NSMutableArray :: < AnyObject > :: new ()]) ; let _ = array . objectAtIndex (0) ; let _ = array . firstObject () . unwrap () ; let _ = array . lastObject () . unwrap () ; }
};
}
