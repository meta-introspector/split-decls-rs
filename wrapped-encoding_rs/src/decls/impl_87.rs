macro_rules! deps {
    () => {
        EncoderResult!();
        EucKrEncoder!();
        Encoder!();
        Encoding!();
        VariantEncoder!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl EucKrEncoder { pub fn new (encoding : & 'static Encoding) -> Encoder { Encoder :: new (encoding , VariantEncoder :: EucKr (EucKrEncoder)) } pub fn max_buffer_length_from_utf16_without_replacement (& self , u16_length : usize ,) -> Option < usize > { u16_length . checked_mul (2) } pub fn max_buffer_length_from_utf8_without_replacement (& self , byte_length : usize ,) -> Option < usize > { byte_length . checked_add (1) } ascii_compatible_bmp_encoder_functions ! ({ let bmp_minus_hangul_start = bmp . wrapping_sub (0xAC00) ; let (lead , trail) = if bmp_minus_hangul_start < (0xD7A4 - 0xAC00) { ksx1001_encode_hangul (bmp , bmp_minus_hangul_start) } else if in_range16 (bmp , 0x33DE , 0xFF01) { if in_range16 (bmp , 0x4E00 , 0x9F9D) || in_range16 (bmp , 0xF900 , 0xFA0C) { if let Some ((hanja_lead , hanja_trail)) = ksx1001_encode_hanja (bmp) { (hanja_lead , hanja_trail) } else { return (EncoderResult :: unmappable_from_bmp (bmp) , source . consumed () , handle . written () ,) ; } } else { return (EncoderResult :: unmappable_from_bmp (bmp) , source . consumed () , handle . written () ,) ; } } else if let Some ((lead , trail)) = ksx1001_encode_misc (bmp) { (lead as u8 , trail as u8) } else { return (EncoderResult :: unmappable_from_bmp (bmp) , source . consumed () , handle . written () ,) ; } ; handle . write_two (lead , trail) } , bmp , self , source , handle , copy_ascii_to_check_space_two , check_space_two , true) ; }
    };
}

impl_87!();