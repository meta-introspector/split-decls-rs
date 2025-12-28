macro_rules! deps {
    () => {
        Variant!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl From < Variant > for EnumVariantId { fn from (def : Variant) -> Self { def . id } }
    };
}

impl_25!();