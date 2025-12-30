// Generated macro for display_debug (function)
macro_rules! Depcrate_tests_stringdisplay_debug {
() => {
// Module: crate::tests::string
// Provides: {"display_debug"}
// Dependencies: {}
# [test] fn display_debug () { let s = NSString :: from_str ("xyz\"123") ; assert_eq ! (format ! ("{s}") , "xyz\"123") ; assert_eq ! (format ! ("{s:?}") , r#""xyz\"123""#) ; }
};
}
