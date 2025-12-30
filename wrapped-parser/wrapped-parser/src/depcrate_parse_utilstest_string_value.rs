// Generated macro for test_string_value (function)
macro_rules! Depcrate_parse_utilstest_string_value {
() => {
// Module: crate::parse::utils
// Provides: {"test_string_value"}
// Dependencies: {}
# [test] fn test_string_value () { assert_eq ! (string_value ("abc") , "abc") ; assert_eq ! (string_value ("\\n\\b\\u2a1A") , "\n\x08\u{2A1A}") ; assert_eq ! (string_value ("\\\"\\\\") , "\"\\") ; }
};
}
