// Generated macro for test_contains (function)
macro_rules! Depcrate_tests_settest_contains {
() => {
// Module: crate::tests::set
// Provides: {"test_contains"}
// Dependencies: {}
# [test] fn test_contains () { let set = NSSet :: < NSString > :: new () ; assert ! (! set . containsObject (ns_string ! ("one"))) ; let set = NSSet :: from_slice (& [ns_string ! ("one") , ns_string ! ("two") , ns_string ! ("two")]) ; assert ! (set . containsObject (ns_string ! ("one"))) ; assert ! (! set . containsObject (ns_string ! ("three"))) ; }
};
}
