macro_rules! deps {
    () => {
        TypeDef!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl Default for TypeDefOrRef { fn default () -> Self { TypeDefOrRef :: TypeDef (id :: TypeDef (u32 :: MAX)) } }
    };
}

impl_141!()