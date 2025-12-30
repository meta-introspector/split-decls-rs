// Generated macro for test_debug (function)
macro_rules! Depcrate_tests_settest_debug {
() => {
// Module: crate::tests::set
// Provides: {"test_debug"}
// Dependencies: {}
# [test] # [allow (clippy :: literal_string_with_formatting_args)] fn test_debug () { let set = NSSet :: < NSObject > :: new () ; assert_eq ! (format ! ("{set:?}") , "{}") ; let set = NSSet :: from_slice (& [ns_string ! ("one") , ns_string ! ("two")]) ; assert ! (matches ! (&* format ! ("{set:?}") , r#"{"one", "two"}"# | r#"{"two", "one"}"# ,)) ; }
};
}
