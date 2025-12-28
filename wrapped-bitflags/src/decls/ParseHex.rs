macro_rules! deps {
    () => {
        ParseError!();
    };
}

macro_rules! ParseHex {
    () => {
        deps!();
        # [doc = "\nParse a value from a hex string.\n"] pub trait ParseHex { # [doc = " Parse the value from hex."] fn parse_hex (input : & str) -> Result < Self , ParseError > where Self : Sized ; }
    };
}

ParseHex!()