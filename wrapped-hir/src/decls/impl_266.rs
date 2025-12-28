macro_rules! deps {
    () => {
        Variant!();
    };
}

macro_rules! impl_266 {
    () => {
        deps!();
        impl From < & Variant > for DefWithBodyId { fn from (& v : & Variant) -> Self { DefWithBodyId :: VariantId (v . into ()) } }
    };
}

impl_266!()