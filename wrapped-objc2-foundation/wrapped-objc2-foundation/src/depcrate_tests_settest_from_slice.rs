// Generated macro for test_from_slice (function)
macro_rules! Depcrate_tests_settest_from_slice {
() => {
// Module: crate::tests::set
// Provides: {"test_from_slice"}
// Dependencies: {}
# [test] fn test_from_slice () { let set = NSSet :: < NSString > :: from_slice (& []) ; assert ! (set . is_empty ()) ; let strs = [ns_string ! ("one") , ns_string ! ("two") , ns_string ! ("three")] ; let set = NSSet :: from_slice (& strs) ; assert ! (strs . into_iter () . all (| s | set . containsObject (s))) ; let nums = [1 , 2 , 3] . map (NSNumber :: new_i32) ; let set = NSSet :: from_retained_slice (& nums) ; assert ! (nums . into_iter () . all (| n | set . containsObject (& n))) ; }
};
}
