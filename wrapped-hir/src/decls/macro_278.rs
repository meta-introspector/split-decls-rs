macro_rules! deps {
    () => {
        Union!();
        VariantDef!();
        Variant!();
        Struct!();
    };
}

macro_rules! macro_278 {
    () => {
        deps!();
        impl_from ! (Struct , Union , Variant for VariantDef) ;
    };
}

macro_278!();