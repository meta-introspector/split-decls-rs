// Generated macro for simd_funcs (module)
macro_rules! Depcratesimd_funcs {
() => {
// Module: crate
// Provides: {"simd_funcs"}
// Dependencies: {}
# [cfg (all (feature = "simd-accel" , any (target_feature = "sse2" , all (target_endian = "little" , target_arch = "aarch64") , all (target_endian = "little" , target_feature = "neon"))))] mod simd_funcs ;
};
}
