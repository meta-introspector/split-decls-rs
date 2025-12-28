macro_rules! deps {
    () => {
        CanonicalCombiningClass!();
    };
}

macro_rules! macro_134 {
    () => {
        deps!();
        make_enumerated_property ! { name : "Canonical_Combining_Class" ; short_name : "ccc" ; ident : CanonicalCombiningClass ; data_marker : crate :: provider :: PropertyEnumCanonicalCombiningClassV1 ; singleton : SINGLETON_PROPERTY_ENUM_CANONICAL_COMBINING_CLASS_V1 ; ule_ty : u8 ; }
    };
}

macro_134!()