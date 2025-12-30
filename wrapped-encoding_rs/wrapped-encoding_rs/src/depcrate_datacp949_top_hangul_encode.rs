// Generated macro for cp949_top_hangul_encode (function)
macro_rules! Depcrate_datacp949_top_hangul_encode {
() => {
// Module: crate::data
// Provides: {"cp949_top_hangul_encode"}
// Dependencies: {}
# [cfg (not (feature = "fast-hangul-encode"))] # [inline (always)] pub fn cp949_top_hangul_encode (bmp : u16) -> u16 { map_with_ranges (& CP949_TOP_HANGUL_OFFSETS [..] , & CP949_TOP_HANGUL_POINTERS [..] , bmp ,) }
};
}
