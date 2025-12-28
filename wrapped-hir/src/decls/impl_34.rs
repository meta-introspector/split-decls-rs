macro_rules! deps {
    () => {
        Variant!();
        VariantDef!();
        Struct!();
        Union!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl From < VariantId > for VariantDef { fn from (def : VariantId) -> Self { match def { VariantId :: StructId (it) => VariantDef :: Struct (it . into ()) , VariantId :: EnumVariantId (it) => VariantDef :: Variant (it . into ()) , VariantId :: UnionId (it) => VariantDef :: Union (it . into ()) , } } }
    };
}

impl_34!()