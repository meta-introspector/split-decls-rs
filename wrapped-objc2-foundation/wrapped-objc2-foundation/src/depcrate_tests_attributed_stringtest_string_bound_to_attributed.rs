// Generated macro for test_string_bound_to_attributed (function)
macro_rules! Depcrate_tests_attributed_stringtest_string_bound_to_attributed {
() => {
// Module: crate::tests::attributed_string
// Provides: {"test_string_bound_to_attributed"}
// Dependencies: {}
# [test] fn test_string_bound_to_attributed () { let attr_s = { let source = NSString :: from_str ("Hello world!") ; NSAttributedString :: from_nsstring (& source) } ; let s = autoreleasepool (| _ | attr_s . string ()) ; assert_eq ! (s . len () , 12) ; }
};
}
