macro_rules! deps {
    () => {
        EnumVariantLoc!();
    };
}

macro_rules! macro_647 {
    () => {
        deps!();
        impl_loc ! (EnumVariantLoc , id : Variant , parent : EnumId) ;
    };
}

macro_647!();