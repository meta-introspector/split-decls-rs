// Generated macro for basic (function)
macro_rules! Depcrate_tests_errorbasic {
() => {
// Module: crate::tests::error
// Provides: {"basic"}
// Dependencies: {}
# [test] fn basic () { let error = NSError :: new (- 999 , unsafe { NSCocoaErrorDomain }) ; let expected = if cfg ! (target_vendor = "apple") { "The operation couldn’t be completed. (Cocoa error -999.)" } else { "NSCocoaErrorDomain -999" } ; assert_eq ! (format ! ("{error}") , expected) ; }
};
}
