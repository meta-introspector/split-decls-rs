// Generated macro for test_is_subset (function)
macro_rules! Depcrate_tests_settest_is_subset {
() => {
// Module: crate::tests::set
// Provides: {"test_is_subset"}
// Dependencies: {}
# [test] fn test_is_subset () { let set1 = NSSet :: from_slice (& [ns_string ! ("one") , ns_string ! ("two")]) ; let set2 = NSSet :: from_slice (& [ns_string ! ("one") , ns_string ! ("two") , ns_string ! ("three")]) ; assert ! (set1 . isSubsetOfSet (& set2)) ; assert ! (! set2 . isSubsetOfSet (& set1)) ; }
};
}
