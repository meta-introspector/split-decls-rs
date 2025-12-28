macro_rules! deps {
    () => {
        AsChar!();
    };
}

macro_rules! impl_319 {
    () => {
        deps!();
        impl AsChar for u8 { # [inline] fn as_char (self) -> char { self as char } # [inline] fn is_alpha (self) -> bool { matches ! (self , 0x41 ..= 0x5A | 0x61 ..= 0x7A) } # [inline] fn is_alphanum (self) -> bool { self . is_alpha () || self . is_dec_digit () } # [inline] fn is_dec_digit (self) -> bool { matches ! (self , 0x30 ..= 0x39) } # [inline] fn is_hex_digit (self) -> bool { matches ! (self , 0x30 ..= 0x39 | 0x41 ..= 0x46 | 0x61 ..= 0x66) } # [inline] fn is_oct_digit (self) -> bool { matches ! (self , 0x30 ..= 0x37) } # [inline] fn is_bin_digit (self) -> bool { matches ! (self , 0x30 ..= 0x31) } # [inline] fn len (self) -> usize { 1 } # [inline] fn is_space (self) -> bool { self == b' ' || self == b'\t' } fn is_newline (self) -> bool { self == b'\n' } }
    };
}

impl_319!()