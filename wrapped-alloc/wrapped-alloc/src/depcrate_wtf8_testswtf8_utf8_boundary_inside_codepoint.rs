// Generated macro for wtf8_utf8_boundary_inside_codepoint (function)
macro_rules! Depcrate_wtf8_testswtf8_utf8_boundary_inside_codepoint {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8_utf8_boundary_inside_codepoint"}
// Dependencies: {}
# [test] # [should_panic (expected = "byte index 1 is not a codepoint boundary")] fn wtf8_utf8_boundary_inside_codepoint () { let string = Wtf8 :: from_str ("é") ; string . check_utf8_boundary (1) ; }
};
}
