// Generated macro for jis0208_kanji_shift_jis_encode (function)
macro_rules! Depcrate_datajis0208_kanji_shift_jis_encode {
() => {
// Module: crate::data
// Provides: {"jis0208_kanji_shift_jis_encode"}
// Dependencies: {}
# [cfg (feature = "fast-kanji-encode")] # [inline (always)] pub fn jis0208_kanji_shift_jis_encode (bmp : u16) -> Option < (u8 , u8) > { let pair = & JIS0208_KANJI_BYTES [bmp as usize - 0x4E00] ; let lead = pair [0] ; let trail = pair [1] ; if lead == 0 && trail == 0 { return None ; } Some ((lead | 0x80 , trail)) }
};
}
