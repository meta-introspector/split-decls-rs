// Generated macro for rot16 (function)
macro_rules! Depcrate_wasm32_simdrot16 {
() => {
// Module: crate::wasm32_simd
// Provides: {"rot16"}
// Dependencies: {}
# [inline (always)] fn rot16 (a : v128) -> v128 { v128_or (u32x4_shr (a , 16) , u32x4_shl (a , 32 - 16)) }
};
}
