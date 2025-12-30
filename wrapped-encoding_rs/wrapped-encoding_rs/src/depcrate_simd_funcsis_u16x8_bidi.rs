// Generated macro for is_u16x8_bidi (function)
macro_rules! Depcrate_simd_funcsis_u16x8_bidi {
() => {
// Module: crate::simd_funcs
// Provides: {"is_u16x8_bidi"}
// Dependencies: {}
# [inline (always)] pub fn is_u16x8_bidi (s : u16x8) -> bool { aarch64_return_false_if_below_hebrew ! (s) ; let below_hebrew = s . simd_lt (u16x8 :: splat (0x0590)) ; non_aarch64_return_false_if_all ! (below_hebrew) ; if all_mask16x8 (below_hebrew | in_range16x8 ! (s , 0x0900 , 0x200F) | in_range16x8 ! (s , 0x2068 , 0xD802) ,) { return false ; } any_mask16x8 ((in_range16x8 ! (s , 0x0590 , 0x0900) | in_range16x8 ! (s , 0xFB1D , 0xFE00) | in_range16x8 ! (s , 0xFE70 , 0xFEFF) | in_range16x8 ! (s , 0xD802 , 0xD804) | in_range16x8 ! (s , 0xD83A , 0xD83C) | s . simd_eq (u16x8 :: splat (0x200F)) | s . simd_eq (u16x8 :: splat (0x202B)) | s . simd_eq (u16x8 :: splat (0x202E)) | s . simd_eq (u16x8 :: splat (0x2067))) ,) }
};
}
