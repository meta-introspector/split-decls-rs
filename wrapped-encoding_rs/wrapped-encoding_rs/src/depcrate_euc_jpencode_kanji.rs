// Generated macro for encode_kanji (function)
macro_rules! Depcrate_euc_jpencode_kanji {
() => {
// Module: crate::euc_jp
// Provides: {"encode_kanji"}
// Dependencies: {}
# [cfg (not (feature = "fast-kanji-encode"))] # [inline (always)] fn encode_kanji (bmp : u16) -> Option < (u8 , u8) > { if 0x4EDD == bmp { Some ((0xA1 , 0xB8)) } else if let Some ((lead , trail)) = jis0208_level1_kanji_euc_jp_encode (bmp) { Some ((lead , trail)) } else if let Some (pos) = jis0208_level2_and_additional_kanji_encode (bmp) { let lead = (pos / 94) + 0xD0 ; let trail = (pos % 94) + 0xA1 ; Some ((lead as u8 , trail as u8)) } else if let Some (pos) = position (& IBM_KANJI [..] , bmp) { let lead = (pos / 94) + 0xF9 ; let trail = (pos % 94) + 0xA1 ; Some ((lead as u8 , trail as u8)) } else { None } }
};
}
