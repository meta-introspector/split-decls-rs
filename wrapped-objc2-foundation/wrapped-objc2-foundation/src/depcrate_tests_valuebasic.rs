// Generated macro for basic (function)
macro_rules! Depcrate_tests_valuebasic {
() => {
// Module: crate::tests::value
// Provides: {"basic"}
// Dependencies: {}
# [test] fn basic () { let val = NSValue :: new (13u32) ; assert_eq ! (unsafe { val . get ::< u32 > () } , 13) ; }
};
}
