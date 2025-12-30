// Generated macro for test_remove (function)
macro_rules! Depcrate_tests_mutable_settest_remove {
() => {
// Module: crate::tests::mutable_set
// Provides: {"test_remove"}
// Dependencies: {}
# [test] fn test_remove () { let strs = [ns_string ! ("one") , ns_string ! ("two") , ns_string ! ("three")] ; let set = NSMutableSet :: from_slice (& strs) ; assert ! (set . containsObject (ns_string ! ("one"))) ; set . removeObject (ns_string ! ("one")) ; assert ! (! set . containsObject (ns_string ! ("one"))) ; assert_eq ! (set . count () , 2) ; set . removeObject (ns_string ! ("one")) ; assert_eq ! (set . count () , 2) ; }
};
}
