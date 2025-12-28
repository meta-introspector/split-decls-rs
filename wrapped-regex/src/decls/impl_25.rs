macro_rules! deps {
    () => {
        Error!();
        Regex!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl core :: str :: FromStr for Regex { type Err = Error ; # [doc = " Attempts to parse a string into a regular expression"] fn from_str (s : & str) -> Result < Regex , Error > { Regex :: new (s) } }
    };
}

impl_25!()