macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! unescape {
    () => {
        deps!();
        fn unescape < W > (w : & mut W , input : & str) -> Result < bool , std :: fmt :: Error > where W : fmt :: Write , { let bytes = input . as_bytes () ; let mut start = 0 ; let mut ptr = 0 ; while let Some (b) = bytes . get (ptr) { if b != & b'\\' { ptr += 1 ; continue ; } if start != ptr { w . write_str (& input [start .. ptr]) ? ; } ptr += 1 ; let new_char = match bytes . get (ptr) { Some (b'\\') => '\\' , Some (b'"') => '"' , Some (u @ b'u') | Some (u @ b'U') => { let seq_start = ptr + 1 ; let len = if u == & b'u' { 4 } else { 6 } ; ptr += len ; encode_unicode (input . get (seq_start .. seq_start + len)) } _ => UNKNOWN_CHAR , } ; ptr += 1 ; w . write_char (new_char) ? ; start = ptr ; } if start == 0 { return Ok (false) ; } if start != ptr { w . write_str (& input [start .. ptr]) ? ; } Ok (true) }
    };
}

unescape!();