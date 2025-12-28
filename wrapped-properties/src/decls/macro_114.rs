macro_rules! deps {
    () => {
        EastAsianWidth!();
    };
}

macro_rules! macro_114 {
    () => {
        deps!();
        make_enumerated_property ! { name : "East_Asian_Width" ; short_name : "ea" ; ident : EastAsianWidth ; data_marker : crate :: provider :: PropertyEnumEastAsianWidthV1 ; singleton : SINGLETON_PROPERTY_ENUM_EAST_ASIAN_WIDTH_V1 ; ule_ty : u8 ; }
    };
}

macro_114!()