// Generated macro for string_and_back (function)
macro_rules! Depcrate_stringstring_and_back {
() => {
// Module: crate::string
// Provides: {"string_and_back"}
// Dependencies: {}
# [test] fn string_and_back () { let original = "The quick brown fox jumped over the slow lazy dog." ; let cfstr = CFString :: from_static_string (original) ; let converted = cfstr . to_string () ; assert_eq ! (converted , original) ; }
};
}
