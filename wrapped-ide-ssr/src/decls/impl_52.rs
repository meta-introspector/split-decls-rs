macro_rules! deps {
    () => {
        SsrError!();
        RawPattern!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl FromStr for RawPattern { type Err = SsrError ; fn from_str (pattern_str : & str) -> Result < RawPattern , SsrError > { Ok (RawPattern { tokens : parse_pattern (pattern_str) ? }) } }
    };
}

impl_52!()