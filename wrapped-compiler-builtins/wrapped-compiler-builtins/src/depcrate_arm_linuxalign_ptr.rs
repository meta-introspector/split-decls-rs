// Generated macro for align_ptr (function)
macro_rules! Depcrate_arm_linuxalign_ptr {
() => {
// Module: crate::arm_linux
// Provides: {"align_ptr"}
// Dependencies: {}
fn align_ptr < T > (ptr : * mut T) -> * mut u32 { let ptr_mask = 3 & (4 - mem :: size_of :: < T > ()) ; (ptr as usize & ! ptr_mask) as * mut u32 }
};
}
