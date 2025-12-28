macro_rules! is_mapped_for_two_byte_encode {
    () => {
        # [allow (clippy :: redundant_pattern_matching , clippy :: if_same_then_else)] fn is_mapped_for_two_byte_encode (bmp : u16) -> bool { let bmp_minus_hiragana = bmp . wrapping_sub (0x3041) ; if bmp_minus_hiragana < 0x53 { true } else if in_inclusive_range16 (bmp , 0x4E00 , 0x9FA0) { is_kanji_mapped (bmp) } else { let bmp_minus_katakana = bmp . wrapping_sub (0x30A1) ; if bmp_minus_katakana < 0x56 { true } else { let bmp_minus_space = bmp . wrapping_sub (0x3000) ; if bmp_minus_space < 3 { true } else if in_inclusive_range16 (bmp , 0xFF61 , 0xFF9F) { true } else if bmp == 0x2212 { true } else if let Some (_) = jis0208_range_encode (bmp) { true } else if in_inclusive_range16 (bmp , 0xFA0E , 0xFA2D) || bmp == 0xF929 || bmp == 0xF9DC { true } else if let Some (_) = ibm_symbol_encode (bmp) { true } else if let Some (_) = jis0208_symbol_encode (bmp) { true } else { false } } } }
    };
}

is_mapped_for_two_byte_encode!();