macro_rules! deps {
    () => {
        EnumVariantLoc!();
    };
}

macro_rules! macro_74 {
    () => {
        deps!();
        impl_intern ! (EnumVariantId , EnumVariantLoc , intern_enum_variant , lookup_intern_enum_variant) ;
    };
}

macro_74!()