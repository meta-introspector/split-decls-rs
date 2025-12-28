macro_rules! deps {
    () => {
        VariantId!();
        AttrDefId!();
    };
}

macro_rules! impl_722 {
    () => {
        deps!();
        impl From < VariantId > for AttrDefId { fn from (vid : VariantId) -> Self { match vid { VariantId :: EnumVariantId (id) => id . into () , VariantId :: StructId (id) => id . into () , VariantId :: UnionId (id) => id . into () , } } }
    };
}

impl_722!();