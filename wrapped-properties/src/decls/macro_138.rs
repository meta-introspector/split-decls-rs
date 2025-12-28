macro_rules! deps {
    () => {
        IndicConjunctBreak!();
    };
}

macro_rules! macro_138 {
    () => {
        deps!();
        make_enumerated_property ! { name : "Indic_Conjunct_Break" ; short_name : "InCB" ; ident : IndicConjunctBreak ; data_marker : crate :: provider :: PropertyEnumIndicConjunctBreakV1 ; singleton : SINGLETON_PROPERTY_ENUM_INDIC_CONJUNCT_BREAK_V1 ; ule_ty : u8 ; }
    };
}

macro_138!()