macro_rules! deps {
    () => {
        HexFloatParseError!();
        Round!();
        Status!();
    };
}

macro_rules! parse_hex_exact {
    () => {
        deps!();
        # [doc = " Parses any float to its bitwise representation, returning an error if it cannot be represented exactly"] pub const fn parse_hex_exact (s : & str , bits : u32 , sig_bits : u32 ,) -> Result < u128 , HexFloatParseError > { match parse_any (s , bits , sig_bits , Round :: Nearest) { Err (e) => Err (e) , Ok ((bits , Status :: OK)) => Ok (bits) , Ok ((_ , status)) if status . overflow () => Err (HexFloatParseError ("the value is too huge")) , Ok ((_ , status)) if status . underflow () => Err (HexFloatParseError ("the value is too tiny")) , Ok ((_ , status)) if status . inexact () => Err (HexFloatParseError ("the value is too precise")) , Ok (_) => unreachable ! () , } }
    };
}

parse_hex_exact!();