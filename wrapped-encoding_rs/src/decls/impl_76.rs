macro_rules! deps {
    () => {
        Encoder!();
        EncoderResult!();
        VariantEncoder!();
        Encoding!();
        EucJpEncoder!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl EucJpEncoder { pub fn new (encoding : & 'static Encoding) -> Encoder { Encoder :: new (encoding , VariantEncoder :: EucJp (EucJpEncoder)) } pub fn max_buffer_length_from_utf16_without_replacement (& self , u16_length : usize ,) -> Option < usize > { u16_length . checked_mul (2) } pub fn max_buffer_length_from_utf8_without_replacement (& self , byte_length : usize ,) -> Option < usize > { byte_length . checked_add (1) } ascii_compatible_bmp_encoder_functions ! ({ let bmp_minus_hiragana = bmp . wrapping_sub (0x3041) ; if bmp_minus_hiragana < 0x53 { handle . write_two (0xA4 , 0xA1 + bmp_minus_hiragana as u8) } else if in_inclusive_range16 (bmp , 0x4E00 , 0x9FA0) { if let Some ((lead , trail)) = encode_kanji (bmp) { handle . write_two (lead , trail) } else { return (EncoderResult :: unmappable_from_bmp (bmp) , source . consumed () , handle . written () ,) ; } } else { let bmp_minus_katakana = bmp . wrapping_sub (0x30A1) ; if bmp_minus_katakana < 0x56 { handle . write_two (0xA5 , 0xA1 + bmp_minus_katakana as u8) } else { let bmp_minus_space = bmp . wrapping_sub (0x3000) ; if bmp_minus_space < 3 { handle . write_two (0xA1 , 0xA1 + bmp_minus_space as u8) } else if bmp == 0xA5 { handle . write_one (0x5Cu8) } else if bmp == 0x203E { handle . write_one (0x7Eu8) } else if in_inclusive_range16 (bmp , 0xFF61 , 0xFF9F) { handle . write_two (0x8Eu8 , (bmp - (0xFF61 - 0xA1)) as u8) } else if bmp == 0x2212 { handle . write_two (0xA1u8 , 0xDDu8) } else if let Some (pointer) = jis0208_range_encode (bmp) { let lead = (pointer / 94) + 0xA1 ; let trail = (pointer % 94) + 0xA1 ; handle . write_two (lead as u8 , trail as u8) } else if in_inclusive_range16 (bmp , 0xFA0E , 0xFA2D) || bmp == 0xF929 || bmp == 0xF9DC { let pos = position (& IBM_KANJI [..] , bmp) . unwrap () ; let lead = (pos / 94) + 0xF9 ; let trail = (pos % 94) + 0xA1 ; handle . write_two (lead as u8 , trail as u8) } else if let Some (pointer) = ibm_symbol_encode (bmp) { let lead = (pointer / 94) + 0xA1 ; let trail = (pointer % 94) + 0xA1 ; handle . write_two (lead as u8 , trail as u8) } else if let Some (pointer) = jis0208_symbol_encode (bmp) { let lead = (pointer / 94) + 0xA1 ; let trail = (pointer % 94) + 0xA1 ; handle . write_two (lead as u8 , trail as u8) } else { return (EncoderResult :: unmappable_from_bmp (bmp) , source . consumed () , handle . written () ,) ; } } } } , bmp , self , source , handle , copy_ascii_to_check_space_two , check_space_two , false) ; }
    };
}

impl_76!()