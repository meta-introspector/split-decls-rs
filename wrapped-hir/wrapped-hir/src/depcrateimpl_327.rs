// Generated macro for impl_327 (impl)
macro_rules! Depcrateimpl_327 {
() => {
// Module: crate
// Provides: {"impl_327"}
// Dependencies: {}
impl From < VariantDef > for ModuleDef { fn from (var : VariantDef) -> Self { match var { VariantDef :: Struct (t) => Adt :: from (t) . into () , VariantDef :: Union (t) => Adt :: from (t) . into () , VariantDef :: Variant (t) => t . into () , } } }
};
}
