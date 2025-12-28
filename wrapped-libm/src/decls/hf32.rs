macro_rules! deps {
    () => {
        HexFloatParseError!();
    };
}

macro_rules! hf32 {
    () => {
        deps!();
        # [doc = " Construct a 32-bit float from hex float representation (C-style)"] # [allow (unused)] pub const fn hf32 (s : & str) -> f32 { match parse_hex_exact (s , 32 , 23) { Ok (bits) => f32_from_bits (bits as u32) , Err (HexFloatParseError (s)) => panic ! ("{}" , s) , } }
    };
}

hf32!();