// Generated macro for wtf8_utf8_boundary_between_surrogates (function)
macro_rules! Depcrate_wtf8_testswtf8_utf8_boundary_between_surrogates {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8_utf8_boundary_between_surrogates"}
// Dependencies: {}
# [test] # [should_panic (expected = "byte index 3 lies between surrogate codepoints")] fn wtf8_utf8_boundary_between_surrogates () { let mut string = Wtf8Buf :: new () ; string . push (CodePoint :: from_u32 (0xD800) . unwrap ()) ; string . push (CodePoint :: from_u32 (0xD800) . unwrap ()) ; string . check_utf8_boundary (3) ; }
};
}
