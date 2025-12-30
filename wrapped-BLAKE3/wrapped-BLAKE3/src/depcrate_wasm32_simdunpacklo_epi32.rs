// Generated macro for unpacklo_epi32 (function)
macro_rules! Depcrate_wasm32_simdunpacklo_epi32 {
() => {
// Module: crate::wasm32_simd
// Provides: {"unpacklo_epi32"}
// Dependencies: {}
# [inline (always)] fn unpacklo_epi32 (a : v128 , b : v128) -> v128 { i32x4_shuffle :: < 0 , 4 , 1 , 5 > (a , b) }
};
}
