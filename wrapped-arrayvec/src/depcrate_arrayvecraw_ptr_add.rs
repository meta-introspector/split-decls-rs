// Generated macro for raw_ptr_add (function)
macro_rules! Depcrate_arrayvecraw_ptr_add {
() => {
// Module: crate::arrayvec
// Provides: {"raw_ptr_add"}
// Dependencies: {}
# [doc = " Rawptr add but uses arithmetic distance for ZST"] unsafe fn raw_ptr_add < T > (ptr : * mut T , offset : usize) -> * mut T { if mem :: size_of :: < T > () == 0 { ptr . cast :: < u8 > () . wrapping_add (offset) . cast :: < T > () } else { ptr . add (offset) } }
};
}
