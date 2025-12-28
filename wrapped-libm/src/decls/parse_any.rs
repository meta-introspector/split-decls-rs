macro_rules! deps {
    () => {
        HexFloatParseError!();
        Status!();
        Round!();
    };
}

macro_rules! parse_any {
    () => {
        deps!();
        # [doc = " Parse any float from hex to its bitwise representation."] pub const fn parse_any (s : & str , bits : u32 , sig_bits : u32 , round : Round ,) -> Result < (u128 , Status) , HexFloatParseError > { let mut b = s . as_bytes () ; if sig_bits > 119 || bits > 128 || bits < sig_bits + 3 || bits > sig_bits + 30 { return Err (HexFloatParseError ("unsupported target float configuration")) ; } let neg = matches ! (b , [b'-' , ..]) ; if let & [b'-' | b'+' , ref rest @ ..] = b { b = rest ; } let sign_bit = 1 << (bits - 1) ; let quiet_bit = 1 << (sig_bits - 1) ; let nan = sign_bit - quiet_bit ; let inf = nan - quiet_bit ; let (mut x , status) = match * b { [b'i' | b'I' , b'n' | b'N' , b'f' | b'F'] => (inf , Status :: OK) , [b'n' | b'N' , b'a' | b'A' , b'n' | b'N'] => (nan , Status :: OK) , [b'0' , b'x' | b'X' , ref rest @ ..] => { let round = match (neg , round) { (true , Round :: Positive) => Round :: Negative , (true , Round :: Negative) => Round :: Positive , (true , Round :: Nearest | Round :: Zero) | (false , _) => round , } ; match parse_finite (rest , bits , sig_bits , round) { Err (e) => return Err (e) , Ok (res) => res , } } _ => return Err (HexFloatParseError ("no hex indicator")) , } ; if neg { x ^= sign_bit ; } Ok ((x , status)) }
    };
}

parse_any!()