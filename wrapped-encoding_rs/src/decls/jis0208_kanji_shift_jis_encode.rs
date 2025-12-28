macro_rules! jis0208_kanji_shift_jis_encode {
    () => {
        # [cfg (feature = "fast-kanji-encode")] # [inline (always)] pub fn jis0208_kanji_shift_jis_encode (bmp : u16) -> Option < (u8 , u8) > { let pair = & JIS0208_KANJI_BYTES [bmp as usize - 0x4E00] ; let lead = pair [0] ; let trail = pair [1] ; if lead == 0 && trail == 0 { return None ; } Some ((lead | 0x80 , trail)) }
    };
}

jis0208_kanji_shift_jis_encode!();