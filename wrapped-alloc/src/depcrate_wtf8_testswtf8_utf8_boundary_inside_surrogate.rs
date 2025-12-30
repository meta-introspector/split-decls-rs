// Generated macro for wtf8_utf8_boundary_inside_surrogate (function)
macro_rules! Depcrate_wtf8_testswtf8_utf8_boundary_inside_surrogate {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8_utf8_boundary_inside_surrogate"}
// Dependencies: {}
# [test] # [should_panic (expected = "byte index 1 is not a codepoint boundary")] fn wtf8_utf8_boundary_inside_surrogate () { let mut string = Wtf8Buf :: new () ; string . push (CodePoint :: from_u32 (0xD800) . unwrap ()) ; string . check_utf8_boundary (1) ; }
};
}
