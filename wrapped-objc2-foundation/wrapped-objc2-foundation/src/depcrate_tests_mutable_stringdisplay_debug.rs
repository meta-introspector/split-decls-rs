// Generated macro for display_debug (function)
macro_rules! Depcrate_tests_mutable_stringdisplay_debug {
() => {
// Module: crate::tests::mutable_string
// Provides: {"display_debug"}
// Dependencies: {}
# [test] fn display_debug () { let s = NSMutableString :: from_str ("test\"123") ; assert_eq ! (format ! ("{s}") , "test\"123") ; assert_eq ! (format ! ("{s:?}") , r#""test\"123""#) ; }
};
}
