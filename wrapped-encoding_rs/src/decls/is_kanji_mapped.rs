macro_rules! is_kanji_mapped {
    () => {
        # [cfg (not (feature = "fast-kanji-encode"))] # [allow (clippy :: redundant_pattern_matching , clippy :: if_same_then_else)] # [inline (always)] fn is_kanji_mapped (bmp : u16) -> bool { if 0x4EDD == bmp { true } else if let Some (_) = jis0208_level1_kanji_shift_jis_encode (bmp) { true } else if let Some (_) = jis0208_level2_and_additional_kanji_encode (bmp) { true } else if let Some (_) = position (& IBM_KANJI [..] , bmp) { true } else { false } }
    };
}

is_kanji_mapped!()