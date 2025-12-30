// Generated macro for test_extend (function)
macro_rules! Depcrate_tests_mutable_settest_extend {
() => {
// Module: crate::tests::mutable_set
// Provides: {"test_extend"}
// Dependencies: {}
# [test] fn test_extend () { let mut set = NSMutableSet :: new () ; assert ! (set . is_empty ()) ; set . extend ([ns_string ! ("one") , ns_string ! ("two") , ns_string ! ("three")]) ; assert_eq ! (set . len () , 3) ; }
};
}
