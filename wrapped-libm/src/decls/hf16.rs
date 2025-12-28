macro_rules! deps {
    () => {
        HexFloatParseError!();
    };
}

macro_rules! hf16 {
    () => {
        deps!();
        # [doc = " Construct a 16-bit float from hex float representation (C-style)"] # [cfg (f16_enabled)] pub const fn hf16 (s : & str) -> f16 { match parse_hex_exact (s , 16 , 10) { Ok (bits) => f16 :: from_bits (bits as u16) , Err (HexFloatParseError (s)) => panic ! ("{}" , s) , } }
    };
}

hf16!()