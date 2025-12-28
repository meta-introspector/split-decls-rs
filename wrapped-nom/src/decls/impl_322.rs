macro_rules! deps {
    () => {
        AsChar!();
    };
}

macro_rules! impl_322 {
    () => {
        deps!();
        impl < 'a > AsChar for & 'a char { # [inline] fn as_char (self) -> char { * self } # [inline] fn is_alpha (self) -> bool { self . is_ascii_alphabetic () } # [inline] fn is_alphanum (self) -> bool { self . is_alpha () || self . is_dec_digit () } # [inline] fn is_dec_digit (self) -> bool { self . is_ascii_digit () } # [inline] fn is_hex_digit (self) -> bool { self . is_ascii_hexdigit () } # [inline] fn is_oct_digit (self) -> bool { self . is_digit (8) } # [inline] fn is_bin_digit (self) -> bool { self . is_digit (2) } # [inline] fn len (self) -> usize { self . len_utf8 () } # [inline] fn is_space (self) -> bool { * self == ' ' || * self == '\t' } fn is_newline (self) -> bool { * self == '\n' } }
    };
}

impl_322!();