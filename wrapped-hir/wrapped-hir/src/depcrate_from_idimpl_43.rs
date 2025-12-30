// Generated macro for impl_43 (impl)
macro_rules! Depcrate_from_idimpl_43 {
() => {
// Module: crate::from_id
// Provides: {"impl_43"}
// Dependencies: {}
impl From < VariantDef > for VariantId { fn from (def : VariantDef) -> Self { match def { VariantDef :: Struct (it) => VariantId :: StructId (it . id) , VariantDef :: Variant (it) => VariantId :: EnumVariantId (it . into ()) , VariantDef :: Union (it) => VariantId :: UnionId (it . id) , } } }
};
}
