// Generated macro for test_clear (function)
macro_rules! Depcrate_tests_mutable_settest_clear {
() => {
// Module: crate::tests::mutable_set
// Provides: {"test_clear"}
// Dependencies: {}
# [test] fn test_clear () { let strs = [ns_string ! ("one") , ns_string ! ("two") , ns_string ! ("three")] ; let set = NSMutableSet :: from_slice (& strs) ; assert_eq ! (set . len () , 3) ; set . removeAllObjects () ; assert ! (set . is_empty ()) ; }
};
}
