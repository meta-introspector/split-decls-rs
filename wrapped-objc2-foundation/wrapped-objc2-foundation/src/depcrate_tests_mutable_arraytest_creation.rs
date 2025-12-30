// Generated macro for test_creation (function)
macro_rules! Depcrate_tests_mutable_arraytest_creation {
() => {
// Module: crate::tests::mutable_array
// Provides: {"test_creation"}
// Dependencies: {}
# [test] # [cfg (feature = "NSValue")] fn test_creation () { use crate :: NSNumber ; let _ = < NSMutableArray < NSNumber > > :: from_retained_slice (& []) ; let _ = NSMutableArray :: from_retained_slice (& [NSNumber :: new_u8 (4) , NSNumber :: new_u8 (2)]) ; let _ = < NSMutableArray < NSNumber > > :: from_slice (& []) ; let _ = NSMutableArray :: from_slice (& [& * NSNumber :: new_u8 (4) , & * NSNumber :: new_u8 (2)]) ; }
};
}
