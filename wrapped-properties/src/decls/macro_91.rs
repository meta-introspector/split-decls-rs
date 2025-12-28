macro_rules! deps {
    () => {
        BidiClass!();
    };
}

macro_rules! macro_91 {
    () => {
        deps!();
        make_enumerated_property ! { name : "Bidi_Class" ; short_name : "bc" ; ident : BidiClass ; data_marker : crate :: provider :: PropertyEnumBidiClassV1 ; singleton : SINGLETON_PROPERTY_ENUM_BIDI_CLASS_V1 ; ule_ty : u8 ; }
    };
}

macro_91!();