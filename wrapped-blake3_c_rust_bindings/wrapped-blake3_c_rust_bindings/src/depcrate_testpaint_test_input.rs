// Generated macro for paint_test_input (function)
macro_rules! Depcrate_testpaint_test_input {
() => {
// Module: crate::test
// Provides: {"paint_test_input"}
// Dependencies: {}
fn paint_test_input (buf : & mut [u8]) { for (i , b) in buf . iter_mut () . enumerate () { * b = (i % 251) as u8 ; } }
};
}
