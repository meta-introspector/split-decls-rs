macro_rules! deps {
    () => {
        Match!();
        PatternID!();
        StateID!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl Match { # [doc = " Return the pattern ID for this match."] pub (crate) fn pattern (& self) -> PatternID { self . pid } # [doc = " Return the ID of the next match."] fn link (& self) -> StateID { self . link } }
    };
}

impl_90!()