// Generated macro for gbk_hanzi_encode (function)
macro_rules! Depcrate_datagbk_hanzi_encode {
() => {
// Module: crate::data
// Provides: {"gbk_hanzi_encode"}
// Dependencies: {}
# [cfg (feature = "fast-gb-hanzi-encode")] # [inline (always)] pub fn gbk_hanzi_encode (bmp_minus_start : u16) -> (u8 , u8) { let pair = & GBK_HANZI_BYTES [bmp_minus_start as usize] ; (pair [0] , pair [1]) }
};
}
