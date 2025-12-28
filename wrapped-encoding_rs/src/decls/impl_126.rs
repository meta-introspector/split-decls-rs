macro_rules! deps {
    () => {
        VariantEncoder!();
        ShiftJisEncoder!();
        Encoding!();
        Encoder!();
        EncoderResult!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl ShiftJisEncoder { pub fn new (encoding : & 'static Encoding) -> Encoder { Encoder :: new (encoding , VariantEncoder :: ShiftJis (ShiftJisEncoder)) } pub fn max_buffer_length_from_utf16_without_replacement (& self , u16_length : usize ,) -> Option < usize > { u16_length . checked_mul (2) } pub fn max_buffer_length_from_utf8_without_replacement (& self , byte_length : usize ,) -> Option < usize > { byte_length . checked_add (1) } ascii_compatible_bmp_encoder_functions ! ({ let bmp_minus_hiragana = bmp . wrapping_sub (0x3041) ; if bmp_minus_hiragana < 0x53 { handle . write_two (0x82 , 0x9F + bmp_minus_hiragana as u8) } else if in_inclusive_range16 (bmp , 0x4E00 , 0x9FA0) { if let Some ((lead , trail)) = encode_kanji (bmp) { handle . write_two (lead , trail) } else { return (EncoderResult :: unmappable_from_bmp (bmp) , source . consumed () , handle . written () ,) ; } } else { let bmp_minus_katakana = bmp . wrapping_sub (0x30A1) ; if bmp_minus_katakana < 0x56 { let trail_offset = if bmp_minus_katakana < 0x3F { 0x40 } else { 0x41 } ; handle . write_two (0x83 , (trail_offset + bmp_minus_katakana) as u8) } else { let bmp_minus_space = bmp . wrapping_sub (0x3000) ; if bmp_minus_space < 3 { handle . write_two (0x81 , 0x40 + bmp_minus_space as u8) } else if bmp == 0xA5 { handle . write_one (0x5Cu8) } else if bmp == 0x80 { handle . write_one (0x80u8) } else if bmp == 0x203E { handle . write_one (0x7Eu8) } else if in_inclusive_range16 (bmp , 0xFF61 , 0xFF9F) { handle . write_one ((bmp - (0xFF61 - 0xA1)) as u8) } else if bmp == 0x2212 { handle . write_two (0x81u8 , 0x7Cu8) } else { let bmp_minus_roman = bmp . wrapping_sub (0x2170) ; let pointer = if bmp_minus_roman <= (0x2179 - 0x2170) { 10716 + bmp_minus_roman as usize } else if let Some (pointer) = jis0208_range_encode (bmp) { pointer } else if in_inclusive_range16 (bmp , 0xFA0E , 0xFA2D) || bmp == 0xF929 || bmp == 0xF9DC { let pos = position (& IBM_KANJI [..] , bmp) . unwrap () ; 10744 + pos } else if let Some (pointer) = jis0208_symbol_encode (bmp) { pointer } else { return (EncoderResult :: unmappable_from_bmp (bmp) , source . consumed () , handle . written () ,) ; } ; let lead = pointer / 188 ; let lead_offset = if lead < 0x1F { 0x81usize } else { 0xC1usize } ; let trail = pointer % 188 ; let trail_offset = if trail < 0x3F { 0x40usize } else { 0x41usize } ; handle . write_two ((lead + lead_offset) as u8 , (trail + trail_offset) as u8) } } } } , bmp , self , source , handle , copy_ascii_to_check_space_two , check_space_two , false) ; }
    };
}

impl_126!();