// Generated macro for gb2312_level2_hanzi_encode (function)
macro_rules! Depcrate_datagb2312_level2_hanzi_encode {
() => {
// Module: crate::data
// Provides: {"gb2312_level2_hanzi_encode"}
// Dependencies: {}
# [cfg (not (feature = "fast-gb-hanzi-encode"))] # [inline (always)] pub fn gb2312_level2_hanzi_encode (bmp : u16) -> Option < usize > { position (& GB2312_HANZI [(94 * (0xD8 - 0xB0)) ..] , bmp) }
};
}
