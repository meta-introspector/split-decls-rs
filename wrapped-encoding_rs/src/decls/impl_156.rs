macro_rules! deps {
    () => {
        VariantEncoder!();
        EncoderResult!();
        Encoder!();
        UserDefinedEncoder!();
        Encoding!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl UserDefinedEncoder { pub fn new (encoding : & 'static Encoding) -> Encoder { Encoder :: new (encoding , VariantEncoder :: UserDefined (UserDefinedEncoder)) } pub fn max_buffer_length_from_utf16_without_replacement (& self , u16_length : usize ,) -> Option < usize > { Some (u16_length) } pub fn max_buffer_length_from_utf8_without_replacement (& self , byte_length : usize ,) -> Option < usize > { Some (byte_length) } encoder_functions ! (eof = { } , body = { if c <= '\u{7F}' { destination_handle . write_one (c as u8) ; continue ; } if c < '\u{F780}' || c > '\u{F7FF}' { return (EncoderResult :: Unmappable (c) , unread_handle . consumed () , destination_handle . written () ,) ; } destination_handle . write_one ((u32 :: from (c) - 0xF700) as u8) ; continue ; } , self = self , src_consumed = src_consumed , source = source , dest = dest , c = c , destination_handle = destination_handle , unread_handle = unread_handle , destination_check = check_space_one) ; }
    };
}

impl_156!();