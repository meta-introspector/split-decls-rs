macro_rules! jis0208_kanji_iso_2022_jp_encode {
    () => {
        # [cfg (feature = "fast-kanji-encode")] # [inline (always)] pub fn jis0208_kanji_iso_2022_jp_encode (bmp : u16) -> Option < (u8 , u8) > { let pair = & JIS0208_KANJI_BYTES [bmp as usize - 0x4E00] ; let lead = pair [0] ; let trail = pair [1] ; if lead == 0 && trail == 0 { return None ; } if lead & 0x80 == 0 { let pos = position (& IBM_KANJI [..] , bmp) . unwrap () ; let lead = (pos / 94) + (0xF9 - 0x80) ; let trail = (pos % 94) + 0x21 ; return Some ((lead as u8 , trail as u8)) ; } Some (shift_jis_to_iso_2022_jp ((lead , trail))) }
    };
}

jis0208_kanji_iso_2022_jp_encode!()