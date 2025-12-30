// Generated macro for Platform (enum)
macro_rules! Depcrate_platformPlatform {
() => {
// Module: crate::platform
// Provides: {"Platform"}
// Dependencies: {}
# [derive (Clone , Copy , Debug)] pub enum Platform { Portable , # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] SSE2 , # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] SSE41 , # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] AVX2 , # [cfg (blake3_avx512_ffi)] # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] AVX512 , # [cfg (blake3_neon)] NEON , # [cfg (blake3_wasm32_simd)] # [allow (non_camel_case_types)] WASM32_SIMD , }
};
}
