macro_rules! deps {
    () => {
        Variant!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl From < EnumVariantId > for Variant { fn from (id : EnumVariantId) -> Self { Variant { id } } }
    };
}

impl_24!();