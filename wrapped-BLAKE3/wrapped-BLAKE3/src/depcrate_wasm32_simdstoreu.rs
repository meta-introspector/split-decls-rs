// Generated macro for storeu (function)
macro_rules! Depcrate_wasm32_simdstoreu {
() => {
// Module: crate::wasm32_simd
// Provides: {"storeu"}
// Dependencies: {}
# [inline (always)] unsafe fn storeu (src : v128 , dest : * mut u8) { unsafe { v128_store (dest as * mut v128 , src) } }
};
}
