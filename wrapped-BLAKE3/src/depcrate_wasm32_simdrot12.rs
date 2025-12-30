// Generated macro for rot12 (function)
macro_rules! Depcrate_wasm32_simdrot12 {
() => {
// Module: crate::wasm32_simd
// Provides: {"rot12"}
// Dependencies: {}
# [inline (always)] fn rot12 (a : v128) -> v128 { v128_or (u32x4_shr (a , 12) , u32x4_shl (a , 32 - 12)) }
};
}
