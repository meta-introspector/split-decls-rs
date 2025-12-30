// Generated macro for match_header_value_char_16_neon (function)
macro_rules! Depcrate_simd_neonmatch_header_value_char_16_neon {
() => {
// Module: crate::simd::neon
// Provides: {"match_header_value_char_16_neon"}
// Dependencies: {}
# [inline] unsafe fn match_header_value_char_16_neon (ptr : * const u8) -> usize { let input = vld1q_u8 (ptr) ; let result = vcleq_u8 (vdupq_n_u8 (b' ') , input) ; let tab = vceqq_u8 (input , vdupq_n_u8 (0x09)) ; let result = vorrq_u8 (result , tab) ; let del = vceqq_u8 (input , vdupq_n_u8 (0x7F)) ; let result = vbicq_u8 (result , del) ; offsetz (result) as usize }
};
}
