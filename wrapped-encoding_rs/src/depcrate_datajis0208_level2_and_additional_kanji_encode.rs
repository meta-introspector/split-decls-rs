// Generated macro for jis0208_level2_and_additional_kanji_encode (function)
macro_rules! Depcrate_datajis0208_level2_and_additional_kanji_encode {
() => {
// Module: crate::data
// Provides: {"jis0208_level2_and_additional_kanji_encode"}
// Dependencies: {}
# [cfg (not (feature = "fast-kanji-encode"))] # [inline (always)] pub fn jis0208_level2_and_additional_kanji_encode (bmp : u16) -> Option < usize > { position (& JIS0208_LEVEL2_AND_ADDITIONAL_KANJI [..] , bmp) }
};
}
