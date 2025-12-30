// Generated macro for rot8 (function)
macro_rules! Depcrate_wasm32_simdrot8 {
() => {
// Module: crate::wasm32_simd
// Provides: {"rot8"}
// Dependencies: {}
# [inline (always)] fn rot8 (a : v128) -> v128 { v128_or (u32x4_shr (a , 8) , u32x4_shl (a , 32 - 8)) }
};
}
