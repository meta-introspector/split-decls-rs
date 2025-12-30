// Generated macro for unpackhi_epi64 (function)
macro_rules! Depcrate_wasm32_simdunpackhi_epi64 {
() => {
// Module: crate::wasm32_simd
// Provides: {"unpackhi_epi64"}
// Dependencies: {}
# [inline (always)] fn unpackhi_epi64 (a : v128 , b : v128) -> v128 { i64x2_shuffle :: < 1 , 3 > (a , b) }
};
}
