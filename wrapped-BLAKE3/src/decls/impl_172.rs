macro_rules! deps {
    () => {
        HexError!();
        Hash!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl core :: str :: FromStr for Hash { type Err = HexError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Hash :: from_hex (s) } }
    };
}

impl_172!()