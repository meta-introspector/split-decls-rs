macro_rules! deps {
    () => {
        EastAsianWidth!();
    };
}

macro_rules! macro_113 {
    () => {
        deps!();
        create_const_array ! { # [allow (missing_docs)] # [allow (non_upper_case_globals)] impl EastAsianWidth { pub const Neutral : EastAsianWidth = EastAsianWidth (0) ; pub const Ambiguous : EastAsianWidth = EastAsianWidth (1) ; pub const Halfwidth : EastAsianWidth = EastAsianWidth (2) ; pub const Fullwidth : EastAsianWidth = EastAsianWidth (3) ; pub const Narrow : EastAsianWidth = EastAsianWidth (4) ; pub const Wide : EastAsianWidth = EastAsianWidth (5) ; } }
    };
}

macro_113!()