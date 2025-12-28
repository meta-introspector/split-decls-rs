macro_rules! deps {
    () => {
        ParsedRule!();
        SsrError!();
        SsrPattern!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl FromStr for SsrPattern { type Err = SsrError ; fn from_str (pattern_str : & str) -> Result < SsrPattern , SsrError > { let raw_pattern = pattern_str . parse () ? ; let parsed_rules = ParsedRule :: new (& raw_pattern , None) ? ; Ok (SsrPattern { parsed_rules }) } }
    };
}

impl_54!();