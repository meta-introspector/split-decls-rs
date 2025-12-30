// Generated macro for match_header_name_char_16_neon (function)
macro_rules! Depcrate_simd_neonmatch_header_name_char_16_neon {
() => {
// Module: crate::simd::neon
// Provides: {"match_header_name_char_16_neon"}
// Dependencies: {}
# [inline] unsafe fn match_header_name_char_16_neon (ptr : * const u8) -> usize { let bitmaps = BITMAPS ; let (bitmap_0_7 , _bitmap_8_15) = bitmaps ; let bitmap_0_7 = vld1q_u8 (bitmap_0_7 . as_ptr ()) ; const BITMASK_LOOKUP_DATA : [u8 ; 16] = [1 , 2 , 4 , 8 , 16 , 32 , 64 , 128 , 1 , 2 , 4 , 8 , 16 , 32 , 64 , 128] ; let bitmask_lookup = vld1q_u8 (BITMASK_LOOKUP_DATA . as_ptr ()) ; let input = vld1q_u8 (ptr) ; let indices_0_7 = vandq_u8 (input , vdupq_n_u8 (0x8F)) ; let row_0_7 = vqtbl1q_u8 (bitmap_0_7 , indices_0_7) ; let bitmask = vqtbl1q_u8 (bitmask_lookup , vshrq_n_u8 (input , 4)) ; let bitsets = row_0_7 ; let tmp = vandq_u8 (bitsets , bitmask) ; let result = vceqq_u8 (tmp , bitmask) ; offsetz (result) as usize }
};
}
