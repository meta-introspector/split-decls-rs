// Generated macro for test_from_utf8_unchecked_sanity (function)
macro_rules! Depcrate_teststest_from_utf8_unchecked_sanity {
() => {
// Module: crate::tests
// Provides: {"test_from_utf8_unchecked_sanity"}
// Dependencies: {}
# [test] fn test_from_utf8_unchecked_sanity () { let text = "hello 🌎, you are nice" ; let compact = unsafe { CompactString :: from_utf8_unchecked (text) } ; assert_eq ! (compact , text) ; }
};
}
