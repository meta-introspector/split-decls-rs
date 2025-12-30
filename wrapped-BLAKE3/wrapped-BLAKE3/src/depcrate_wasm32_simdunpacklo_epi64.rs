// Generated macro for unpacklo_epi64 (function)
macro_rules! Depcrate_wasm32_simdunpacklo_epi64 {
() => {
// Module: crate::wasm32_simd
// Provides: {"unpacklo_epi64"}
// Dependencies: {}
# [inline (always)] fn unpacklo_epi64 (a : v128 , b : v128) -> v128 { i64x2_shuffle :: < 0 , 2 > (a , b) }
};
}
