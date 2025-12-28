macro_rules! deps {
    () => {
        EscapeError!();
    };
}

macro_rules! hex_escape {
    () => {
        deps!();
        # [doc = " Interpret a hexadecimal escape"] # [doc = ""] # [doc = " Parses the two hexadecimal characters of a hexadecimal escape without the leading r\"\\x\"."] # [inline] fn hex_escape (chars : & mut impl Iterator < Item = char >) -> Result < u8 , EscapeError > { let hi = chars . next () . ok_or (EscapeError :: TooShortHexEscape) ? ; let hi = hi . to_digit (16) . ok_or (EscapeError :: InvalidCharInHexEscape) ? ; let lo = chars . next () . ok_or (EscapeError :: TooShortHexEscape) ? ; let lo = lo . to_digit (16) . ok_or (EscapeError :: InvalidCharInHexEscape) ? ; Ok ((hi * 16 + lo) as u8) }
    };
}

hex_escape!()