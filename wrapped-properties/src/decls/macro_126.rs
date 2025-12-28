macro_rules! deps {
    () => {
        WordBreak!();
    };
}

macro_rules! macro_126 {
    () => {
        deps!();
        make_enumerated_property ! { name : "Word_Break" ; short_name : "WB" ; ident : WordBreak ; data_marker : crate :: provider :: PropertyEnumWordBreakV1 ; singleton : SINGLETON_PROPERTY_ENUM_WORD_BREAK_V1 ; ule_ty : u8 ; }
    };
}

macro_126!();