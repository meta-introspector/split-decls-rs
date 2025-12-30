// Generated macro for jis0208_level1_kanji_shift_jis_encode (function)
macro_rules! Depcrate_datajis0208_level1_kanji_shift_jis_encode {
() => {
// Module: crate::data
// Provides: {"jis0208_level1_kanji_shift_jis_encode"}
// Dependencies: {}
# [cfg (all (feature = "less-slow-kanji-encode" , not (feature = "fast-kanji-encode")))] # [inline (always)] pub fn jis0208_level1_kanji_shift_jis_encode (bmp : u16) -> Option < (u8 , u8) > { match JIS0208_LEVEL1_KANJI_CODE_POINTS . binary_search (& bmp) { Ok (i) => { let pair = & JIS0208_LEVEL1_KANJI_SHIFT_JIS_BYTES [i] ; Some ((pair [0] , pair [1])) } Err (_) => None , } }
};
}
