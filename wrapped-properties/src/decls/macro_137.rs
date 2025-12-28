macro_rules! deps {
    () => {
        IndicConjunctBreak!();
    };
}

macro_rules! macro_137 {
    () => {
        deps!();
        create_const_array ! { # [doc (hidden)] # [allow (non_upper_case_globals)] impl IndicConjunctBreak { pub const None : IndicConjunctBreak = IndicConjunctBreak (0) ; pub const Consonant : IndicConjunctBreak = IndicConjunctBreak (1) ; pub const Extend : IndicConjunctBreak = IndicConjunctBreak (2) ; pub const Linker : IndicConjunctBreak = IndicConjunctBreak (3) ; } }
    };
}

macro_137!()