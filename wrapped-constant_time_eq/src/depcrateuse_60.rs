// Generated macro for use_60 (use)
macro_rules! Depcrateuse_60 {
() => {
// Module: crate
// Provides: {"use_60"}
// Dependencies: {}
# [cfg (not (any (all (any (target_arch = "x86" , target_arch = "x86_64") , target_feature = "sse2" , not (miri)) , all (target_arch = "aarch64" , target_feature = "neon" , not (miri)))))] use generic as simd ;
};
}
