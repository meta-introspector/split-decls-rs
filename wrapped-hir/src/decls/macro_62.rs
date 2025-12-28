macro_rules! deps {
    () => {
        Variant!();
        VariantDef!();
        Struct!();
        Union!();
    };
}

macro_rules! macro_62 {
    () => {
        deps!();
        impl_from ! (Struct , Union , Variant for VariantDef) ;
    };
}

macro_62!()