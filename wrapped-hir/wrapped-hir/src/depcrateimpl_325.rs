// Generated macro for impl_325 (impl)
macro_rules! Depcrateimpl_325 {
() => {
// Module: crate
// Provides: {"impl_325"}
// Dependencies: {}
impl From < VariantDef > for ModuleDef { fn from (var : VariantDef) -> Self { match var { VariantDef :: Struct (t) => Adt :: from (t) . into () , VariantDef :: Union (t) => Adt :: from (t) . into () , VariantDef :: Variant (t) => t . into () , } } }
};
}
