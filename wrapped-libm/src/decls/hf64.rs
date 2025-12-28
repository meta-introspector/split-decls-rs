macro_rules! deps {
    () => {
        HexFloatParseError!();
    };
}

macro_rules! hf64 {
    () => {
        deps!();
        # [doc = " Construct a 64-bit float from hex float representation (C-style)"] pub const fn hf64 (s : & str) -> f64 { match parse_hex_exact (s , 64 , 52) { Ok (bits) => f64_from_bits (bits as u64) , Err (HexFloatParseError (s)) => panic ! ("{}" , s) , } }
    };
}

hf64!();