macro_rules! deps {
    () => {
        Definition!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl From < VariantDef > for Definition { fn from (def : VariantDef) -> Self { ModuleDef :: from (def) . into () } }
    };
}

impl_44!();