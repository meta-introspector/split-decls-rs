macro_rules! jis0208_level1_kanji_euc_jp_encode {
    () => {
        # [cfg (all (feature = "less-slow-kanji-encode" , not (feature = "fast-kanji-encode")))] # [inline (always)] pub fn jis0208_level1_kanji_euc_jp_encode (bmp : u16) -> Option < (u8 , u8) > { jis0208_level1_kanji_shift_jis_encode (bmp) . map (shift_jis_to_euc_jp) }
    };
}

jis0208_level1_kanji_euc_jp_encode!();