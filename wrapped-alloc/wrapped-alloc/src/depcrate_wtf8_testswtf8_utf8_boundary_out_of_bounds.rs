// Generated macro for wtf8_utf8_boundary_out_of_bounds (function)
macro_rules! Depcrate_wtf8_testswtf8_utf8_boundary_out_of_bounds {
() => {
// Module: crate::wtf8::tests
// Provides: {"wtf8_utf8_boundary_out_of_bounds"}
// Dependencies: {}
# [test] # [should_panic (expected = "byte index 4 is out of bounds")] fn wtf8_utf8_boundary_out_of_bounds () { let string = Wtf8 :: from_str ("aé") ; string . check_utf8_boundary (4) ; }
};
}
