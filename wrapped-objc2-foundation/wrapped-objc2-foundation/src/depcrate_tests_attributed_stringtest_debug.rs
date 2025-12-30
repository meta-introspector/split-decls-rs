// Generated macro for test_debug (function)
macro_rules! Depcrate_tests_attributed_stringtest_debug {
() => {
// Module: crate::tests::attributed_string
// Provides: {"test_debug"}
// Dependencies: {}
# [test] # [cfg (feature = "NSDictionary")] fn test_debug () { let s = NSAttributedString :: from_nsstring (ns_string ! ("abc")) ; let expected = if cfg ! (feature = "gnustep-1-7") { "abc{}" } else { "abc{\n}" } ; assert_eq ! (format ! ("{s:?}") , expected) ; let obj = NSObject :: new () . into_super () ; let ptr : * const AnyObject = & * obj ; let s = unsafe { NSAttributedString :: new_with_attributes (ns_string ! ("abc") , & crate :: NSDictionary :: from_retained_objects (& [ns_string ! ("test")] , & [obj]) ,) } ; let expected = if cfg ! (feature = "gnustep-1-7") { format ! ("abc{{test = \"<NSObject: {ptr:?}>\"; }}") } else { format ! ("abc{{\n    test = \"<NSObject: {ptr:?}>\";\n}}") } ; assert_eq ! (format ! ("{s:?}") , expected) ; }
};
}
