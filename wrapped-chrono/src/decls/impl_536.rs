macro_rules! deps {
    () => {
        FixedOffset!();
        ParseError!();
    };
}

macro_rules! impl_536 {
    () => {
        deps!();
        # [doc = " Parsing a `str` into a `FixedOffset` uses the format [`%z`](crate::format::strftime)."] impl FromStr for FixedOffset { type Err = ParseError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { let (_ , offset) = scan :: timezone_offset (s , scan :: colon_or_space , false , false , true) ? ; Self :: east_opt (offset) . ok_or (OUT_OF_RANGE) } }
    };
}

impl_536!();