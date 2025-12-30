// Generated macro for neon (module)
macro_rules! Depcrate_simdneon {
() => {
// Module: crate::simd
// Provides: {"neon"}
// Dependencies: {}
# [cfg (all (not (any (httparse_disable_simd , miri)) , target_arch = "aarch64" , target_feature = "neon" ,))] mod neon ;
};
}
