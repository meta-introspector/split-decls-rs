macro_rules! deps {
    () => {
        SentenceBreak!();
    };
}

macro_rules! macro_130 {
    () => {
        deps!();
        make_enumerated_property ! { name : "Sentence_Break" ; short_name : "SB" ; ident : SentenceBreak ; data_marker : crate :: provider :: PropertyEnumSentenceBreakV1 ; singleton : SINGLETON_PROPERTY_ENUM_SENTENCE_BREAK_V1 ; ule_ty : u8 ; }
    };
}

macro_130!();