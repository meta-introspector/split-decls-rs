macro_rules! deps {
    () => {
        Struct!();
        Variant!();
        Union!();
        VariantDef!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl From < VariantDef > for VariantId { fn from (def : VariantDef) -> Self { match def { VariantDef :: Struct (it) => VariantId :: StructId (it . id) , VariantDef :: Variant (it) => VariantId :: EnumVariantId (it . into ()) , VariantDef :: Union (it) => VariantId :: UnionId (it . id) , } } }
    };
}

impl_35!();