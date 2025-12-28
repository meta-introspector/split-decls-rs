macro_rules! deps {
    () => {
        Variant!();
        Struct!();
        Union!();
        VariantDef!();
    };
}

macro_rules! macro_278 {
    () => {
        deps!();
        impl_from ! (Struct , Union , Variant for VariantDef) ;
    };
}

macro_278!()