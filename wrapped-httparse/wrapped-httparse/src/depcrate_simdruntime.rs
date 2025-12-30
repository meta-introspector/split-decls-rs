// Generated macro for runtime (module)
macro_rules! Depcrate_simdruntime {
() => {
// Module: crate::simd
// Provides: {"runtime"}
// Dependencies: {}
# [cfg (all (not (any (httparse_disable_simd , miri)) , feature = "std" , not (any (target_feature = "sse4.2" , target_feature = "avx2" ,)) , any (target_arch = "x86" , target_arch = "x86_64" ,) ,))] mod runtime ;
};
}
