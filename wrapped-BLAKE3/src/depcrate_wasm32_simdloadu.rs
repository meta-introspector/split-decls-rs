// Generated macro for loadu (function)
macro_rules! Depcrate_wasm32_simdloadu {
() => {
// Module: crate::wasm32_simd
// Provides: {"loadu"}
// Dependencies: {}
# [inline (always)] unsafe fn loadu (src : * const u8) -> v128 { unsafe { v128_load (src as * const v128) } }
};
}
