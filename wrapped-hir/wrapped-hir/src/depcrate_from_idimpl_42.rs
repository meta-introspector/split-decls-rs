// Generated macro for impl_42 (impl)
macro_rules! Depcrate_from_idimpl_42 {
() => {
// Module: crate::from_id
// Provides: {"impl_42"}
// Dependencies: {}
impl From < VariantId > for VariantDef { fn from (def : VariantId) -> Self { match def { VariantId :: StructId (it) => VariantDef :: Struct (it . into ()) , VariantId :: EnumVariantId (it) => VariantDef :: Variant (it . into ()) , VariantId :: UnionId (it) => VariantDef :: Union (it . into ()) , } } }
};
}
