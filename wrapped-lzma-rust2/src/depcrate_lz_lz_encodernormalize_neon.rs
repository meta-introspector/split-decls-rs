// Generated macro for normalize_neon (function)
macro_rules! Depcrate_lz_lz_encodernormalize_neon {
() => {
// Module: crate::lz::lz_encoder
// Provides: {"normalize_neon"}
// Dependencies: {}
# [doc = " Normalization implementation using ARM NEON for 128-bit SIMD processing."] # [cfg (all (feature = "std" , feature = "optimization" , target_arch = "aarch64"))] # [target_feature (enable = "neon")] unsafe fn normalize_neon (positions : & mut [i32] , norm_offset : i32) { use core :: arch :: aarch64 :: * ; let norm_v = vdupq_n_s32 (norm_offset) ; let (prefix , chunks , suffix) = positions . align_to_mut :: < int32x4_t > () ; normalize_scalar (prefix , norm_offset) ; for chunk in chunks { let ptr = chunk as * mut int32x4_t as * mut i32 ; let data = vld1q_s32 (ptr) ; let max_val = vmaxq_s32 (data , norm_v) ; let result = vsubq_s32 (max_val , norm_v) ; vst1q_s32 (ptr , result) ; } normalize_scalar (suffix , norm_offset) ; }
};
}
