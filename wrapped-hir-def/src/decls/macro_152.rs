macro_rules! deps {
    () => {
        VariantId!();
    };
}

macro_rules! macro_152 {
    () => {
        deps!();
        impl_from ! (EnumVariantId , StructId , UnionId for VariantId) ;
    };
}

macro_152!()