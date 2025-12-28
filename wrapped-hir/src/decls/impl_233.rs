macro_rules! deps {
    () => {
        Adt!();
        ModuleDef!();
        VariantDef!();
        Struct!();
        Union!();
        Variant!();
    };
}

macro_rules! impl_233 {
    () => {
        deps!();
        impl From < VariantDef > for ModuleDef { fn from (var : VariantDef) -> Self { match var { VariantDef :: Struct (t) => Adt :: from (t) . into () , VariantDef :: Union (t) => Adt :: from (t) . into () , VariantDef :: Variant (t) => t . into () , } } }
    };
}

impl_233!();