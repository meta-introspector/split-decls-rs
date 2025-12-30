// Generated macro for test_insert (function)
macro_rules! Depcrate_tests_mutable_settest_insert {
() => {
// Module: crate::tests::mutable_set
// Provides: {"test_insert"}
// Dependencies: {}
# [test] fn test_insert () { let set = NSMutableSet :: new () ; assert ! (set . is_empty ()) ; set . addObject (ns_string ! ("one")) ; set . addObject (ns_string ! ("one")) ; assert_eq ! (set . count () , 1) ; set . addObject (ns_string ! ("two")) ; assert_eq ! (set . count () , 2) ; }
};
}
