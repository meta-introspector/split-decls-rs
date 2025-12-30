// Generated macro for test_creation (function)
macro_rules! Depcrate_tests_arraytest_creation {
() => {
// Module: crate::tests::array
// Provides: {"test_creation"}
// Dependencies: {}
# [test] fn test_creation () { let _ = < NSArray < NSNumber > > :: from_retained_slice (& []) ; let _ = NSArray :: from_retained_slice (& [NSNumber :: new_u8 (4) , NSNumber :: new_u8 (2)]) ; let _ = < NSArray < NSNumber > > :: from_slice (& []) ; let _ = NSArray :: from_slice (& [& * NSNumber :: new_u8 (4) , & * NSNumber :: new_u8 (2)]) ; }
};
}
