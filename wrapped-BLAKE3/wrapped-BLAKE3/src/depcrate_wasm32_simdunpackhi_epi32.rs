// Generated macro for unpackhi_epi32 (function)
macro_rules! Depcrate_wasm32_simdunpackhi_epi32 {
() => {
// Module: crate::wasm32_simd
// Provides: {"unpackhi_epi32"}
// Dependencies: {}
# [inline (always)] fn unpackhi_epi32 (a : v128 , b : v128) -> v128 { i32x4_shuffle :: < 2 , 6 , 3 , 7 > (a , b) }
};
}
