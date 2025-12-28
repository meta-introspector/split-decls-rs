macro_rules! deps {
    () => {
        Result!();
        Writer!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl < W : fmt :: Write > Writer < W > { fn write_literal_char (& mut self , c : char) -> fmt :: Result { if is_meta_character (c) { self . wtr . write_str ("\\") ? ; } self . wtr . write_char (c) } fn write_literal_byte (& mut self , b : u8) -> fmt :: Result { if b <= 0x7F && ! b . is_ascii_control () && ! b . is_ascii_whitespace () { self . write_literal_char (char :: try_from (b) . unwrap ()) } else { write ! (self . wtr , "(?-u:\\x{b:02X})") } } fn write_literal_class_byte (& mut self , b : u8) -> fmt :: Result { if b <= 0x7F && ! b . is_ascii_control () && ! b . is_ascii_whitespace () { self . write_literal_char (char :: try_from (b) . unwrap ()) } else { write ! (self . wtr , "\\x{b:02X}") } } }
    };
}

impl_184!();