// Generated macro for test_insert_number (function)
macro_rules! Depcrate_tests_mutable_settest_insert_number {
() => {
// Module: crate::tests::mutable_set
// Provides: {"test_insert_number"}
// Dependencies: {}
# [test] # [cfg (feature = "NSValue")] fn test_insert_number () { use crate :: NSNumber ; let set = NSMutableSet :: new () ; set . addObject (& * NSNumber :: new_u32 (42)) ; set . addObject (& * NSNumber :: new_u32 (42)) ; assert_eq ! (set . count () , 1) ; }
};
}
