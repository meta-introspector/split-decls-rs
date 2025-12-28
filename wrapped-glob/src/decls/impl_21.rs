macro_rules! deps {
    () => {
        Pattern!();
        PatternError!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl FromStr for Pattern { type Err = PatternError ; fn from_str (s : & str) -> Result < Self , PatternError > { Self :: new (s) } }
    };
}

impl_21!();