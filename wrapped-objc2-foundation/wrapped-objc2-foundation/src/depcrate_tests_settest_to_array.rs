// Generated macro for test_to_array (function)
macro_rules! Depcrate_tests_settest_to_array {
() => {
// Module: crate::tests::set
// Provides: {"test_to_array"}
// Dependencies: {}
# [test] # [cfg (feature = "NSArray")] fn test_to_array () { let nums = [1 , 2 , 3] ; let set = NSSet :: from_retained_slice (& nums . map (NSNumber :: new_i32)) ; assert_eq ! (set . allObjects () . len () , 3) ; assert ! (set . allObjects () . iter () . all (| i | nums . contains (& i . as_i32 ()))) ; }
};
}
