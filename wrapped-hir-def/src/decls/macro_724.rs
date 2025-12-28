macro_rules! deps {
    () => {
        VariantId!();
    };
}

macro_rules! macro_724 {
    () => {
        deps!();
        impl_from ! (EnumVariantId , StructId , UnionId for VariantId) ;
    };
}

macro_724!();