// Generated macro for default (module)
macro_rules! Depcrate_blockdefault {
() => {
// Module: crate::block
// Provides: {"default"}
// Dependencies: {}
# [cfg (all (not (all (target_family = "wasm" , target_feature = "simd128")) , not (target_feature = "sse2") , not (target_feature = "avx") , not (target_feature = "avx2") ,))] mod default ;
};
}
