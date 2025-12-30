// Generated macro for test_debug (function)
macro_rules! Depcrate_tests_dictionarytest_debug {
() => {
// Module: crate::tests::dictionary
// Provides: {"test_debug"}
// Dependencies: {}
# [test] fn test_debug () { let key = ns_string ! ("a") ; let val = ns_string ! ("b") ; let dict = NSDictionary :: from_slices (& [key] , & [val]) ; assert_eq ! (format ! ("{dict:?}") , r#"{"a": "b"}"#) ; }
};
}
