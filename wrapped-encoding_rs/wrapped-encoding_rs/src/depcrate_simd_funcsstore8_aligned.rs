// Generated macro for store8_aligned (function)
macro_rules! Depcrate_simd_funcsstore8_aligned {
() => {
// Module: crate::simd_funcs
// Provides: {"store8_aligned"}
// Dependencies: {}
# [doc = " Safety invariant: ptr must be valid for an aligned-for-u16x8 store of 16 bytes"] # [allow (dead_code)] # [inline (always)] pub unsafe fn store8_aligned (ptr : * mut u16 , s : u16x8) { * (ptr as * mut u16x8) = s ; }
};
}
