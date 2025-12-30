// Generated macro for match_url_char_16_neon (function)
macro_rules! Depcrate_simd_neonmatch_url_char_16_neon {
() => {
// Module: crate::simd::neon
// Provides: {"match_url_char_16_neon"}
// Dependencies: {}
# [inline] unsafe fn match_url_char_16_neon (ptr : * const u8) -> usize { let input = vld1q_u8 (ptr) ; let result = vcleq_u8 (vdupq_n_u8 (b'!') , input) ; let del = vceqq_u8 (input , vdupq_n_u8 (0x7F)) ; let result = vbicq_u8 (result , del) ; offsetz (result) as usize }
};
}
