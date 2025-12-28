macro_rules! deps {
    () => {
        Struct!();
        Variant!();
        Union!();
        VariantDef!();
        ModuleDef!();
        Adt!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl From < VariantDef > for ModuleDef { fn from (var : VariantDef) -> Self { match var { VariantDef :: Struct (t) => Adt :: from (t) . into () , VariantDef :: Union (t) => Adt :: from (t) . into () , VariantDef :: Variant (t) => t . into () , } } }
    };
}

impl_17!()