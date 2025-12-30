// Generated macro for store16_aligned (function)
macro_rules! Depcrate_simd_funcsstore16_aligned {
() => {
// Module: crate::simd_funcs
// Provides: {"store16_aligned"}
// Dependencies: {}
# [doc = " Safety invariant: ptr must be valid for an aligned-for-u8x16 store of 16 bytes"] # [allow (dead_code)] # [inline (always)] pub unsafe fn store16_aligned (ptr : * mut u8 , s : u8x16) { * (ptr as * mut u8x16) = s ; }
};
}
