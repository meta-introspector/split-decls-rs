macro_rules! deps {
    () => {
        HexNibbles!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < 's > HexNibbles < 's > { # [doc = " Decode an integer value (with the \"most significant nibble\" first),"] # [doc = " returning `None` if it can't fit in an `u64`."] fn try_parse_uint (& self) -> Option < u64 > { let nibbles = self . nibbles . trim_start_matches ("0") ; if nibbles . len () > 16 { return None ; } let mut v = 0 ; for nibble in nibbles . chars () { v = (v << 4) | (nibble . to_digit (16) . unwrap () as u64) ; } Some (v) } # [doc = " Decode a UTF-8 byte sequence (with each byte using a pair of nibbles)"] # [doc = " into individual `char`s, returning `None` for invalid UTF-8."] fn try_parse_str_chars (& self) -> Option < impl Iterator < Item = char > + 's > { if self . nibbles . len () % 2 != 0 { return None ; } let mut bytes = self . nibbles . as_bytes () . chunks_exact (2) . map (| slice | match slice { [a , b] => [a , b] , _ => unreachable ! () , }) . map (| [& hi , & lo] | { let half = | nibble : u8 | (nibble as char) . to_digit (16) . unwrap () as u8 ; (half (hi) << 4) | half (lo) }) ; let chars = iter :: from_fn (move | | { bytes . next () . map (| first_byte | -> Result < char , () > { enum Utf8FirstByteError { ContinuationByte , TooLong , } fn utf8_len_from_first_byte (byte : u8) -> Result < usize , Utf8FirstByteError > { match byte { 0x00 ..= 0x7f => Ok (1) , 0x80 ..= 0xbf => Err (Utf8FirstByteError :: ContinuationByte) , 0xc0 ..= 0xdf => Ok (2) , 0xe0 ..= 0xef => Ok (3) , 0xf0 ..= 0xf7 => Ok (4) , 0xf8 ..= 0xff => Err (Utf8FirstByteError :: TooLong) , } } let utf8_len = utf8_len_from_first_byte (first_byte) . map_err (| _ | ()) ? ; let utf8 = & mut [first_byte , 0 , 0 , 0] [.. utf8_len] ; for i in 1 .. utf8_len { utf8 [i] = bytes . next () . ok_or (()) ? ; } let s = str :: from_utf8 (utf8) . map_err (| _ | ()) ? ; let mut chars = s . chars () ; match (chars . next () , chars . next ()) { (Some (c) , None) => Ok (c) , _ => unreachable ! ("str::from_utf8({:?}) = {:?} was expected to have 1 char, \
                         but {} chars were found" , utf8 , s , s . chars () . count ()) , } }) }) ; if chars . clone () . any (| r | r . is_err ()) { None } else { Some (chars . map (Result :: unwrap)) } } }
    };
}

impl_20!();