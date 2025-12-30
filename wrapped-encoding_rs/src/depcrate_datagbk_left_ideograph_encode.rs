// Generated macro for gbk_left_ideograph_encode (function)
macro_rules! Depcrate_datagbk_left_ideograph_encode {
() => {
// Module: crate::data
// Provides: {"gbk_left_ideograph_encode"}
// Dependencies: {}
# [cfg (not (feature = "fast-gb-hanzi-encode"))] # [inline (always)] pub fn gbk_left_ideograph_encode (bmp : u16) -> u16 { map_with_ranges (& GBK_LEFT_IDEOGRAPH_OFFSETS [..] , & GBK_LEFT_IDEOGRAPH_POINTERS [..] , bmp ,) }
};
}
