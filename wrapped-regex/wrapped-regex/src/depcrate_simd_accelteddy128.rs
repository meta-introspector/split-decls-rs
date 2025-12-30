// Generated macro for teddy128 (module)
macro_rules! Depcrate_simd_accelteddy128 {
() => {
// Module: crate::simd_accel
// Provides: {"teddy128"}
// Dependencies: {}
# [cfg (not (target_feature = "ssse3"))] # [path = "../simd_fallback/teddy128.rs"] pub mod teddy128 ;
};
}
