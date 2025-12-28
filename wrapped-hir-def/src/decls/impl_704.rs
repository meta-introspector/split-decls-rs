macro_rules! deps {
    () => {
        DefWithBodyId!();
        VariantId!();
    };
}

macro_rules! impl_704 {
    () => {
        deps!();
        impl From < EnumVariantId > for DefWithBodyId { fn from (id : EnumVariantId) -> Self { DefWithBodyId :: VariantId (id) } }
    };
}

impl_704!()