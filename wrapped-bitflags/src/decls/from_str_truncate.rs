macro_rules! deps {
    () => {
        Flags!();
        ParseError!();
        Bits!();
        ParseHex!();
    };
}

macro_rules! from_str_truncate {
    () => {
        deps!();
        # [doc = "\nParse a flags value from text.\n\nThis function will fail on any names that don't correspond to defined flags.\nUnknown bits will be ignored.\n"] pub fn from_str_truncate < B : Flags > (input : & str) -> Result < B , ParseError > where B :: Bits : ParseHex , { Ok (B :: from_bits_truncate (from_str :: < B > (input) ? . bits ())) }
    };
}

from_str_truncate!()