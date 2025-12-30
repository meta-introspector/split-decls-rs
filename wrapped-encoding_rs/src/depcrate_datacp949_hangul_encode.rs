// Generated macro for cp949_hangul_encode (function)
macro_rules! Depcrate_datacp949_hangul_encode {
() => {
// Module: crate::data
// Provides: {"cp949_hangul_encode"}
// Dependencies: {}
# [cfg (feature = "fast-hangul-encode")] # [inline (always)] pub fn cp949_hangul_encode (bmp_minus_start : u16) -> (u8 , u8) { let pair = & CP949_HANGUL_BYTES [bmp_minus_start as usize] ; (pair [0] , pair [1]) }
};
}
