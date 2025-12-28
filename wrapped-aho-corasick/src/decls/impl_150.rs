macro_rules! deps {
    () => {
        Match!();
        PatternID!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl Match { # [doc = " Returns the ID of the pattern that matched."] pub (crate) fn pattern (& self) -> PatternID { self . pid } # [doc = " Returns a pointer into the haystack at which the match starts."] pub (crate) fn start (& self) -> * const u8 { self . start } # [doc = " Returns a pointer into the haystack at which the match ends."] pub (crate) fn end (& self) -> * const u8 { self . end } }
    };
}

impl_150!()