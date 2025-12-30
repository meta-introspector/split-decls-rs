// Generated macro for c_string_length (function)
macro_rules! Depcrate_mem_implsc_string_length {
() => {
// Module: crate::mem::impls
// Provides: {"c_string_length"}
// Dependencies: {}
# [inline (always)] pub unsafe fn c_string_length (mut s : * const core :: ffi :: c_char) -> usize { let mut n = 0 ; while * s != 0 { n += 1 ; s = s . wrapping_add (1) ; } n }
};
}
