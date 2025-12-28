macro_rules! deps {
    () => {
        HangulSyllableType!();
    };
}

macro_rules! macro_110 {
    () => {
        deps!();
        make_enumerated_property ! { name : "Hangul_Syllable_Type" ; short_name : "hst" ; ident : HangulSyllableType ; data_marker : crate :: provider :: PropertyEnumHangulSyllableTypeV1 ; singleton : SINGLETON_PROPERTY_ENUM_HANGUL_SYLLABLE_TYPE_V1 ; ule_ty : u8 ; }
    };
}

macro_110!()