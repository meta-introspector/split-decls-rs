// Generated macro for impl_65 (impl)
macro_rules! Depcrate_has_sourceimpl_65 {
() => {
// Module: crate::has_source
// Provides: {"impl_65"}
// Dependencies: {}
impl HasSource for VariantDef { type Ast = ast :: VariantDef ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { match self { VariantDef :: Struct (s) => Some (s . source (db) ? . map (ast :: VariantDef :: Struct)) , VariantDef :: Union (u) => Some (u . source (db) ? . map (ast :: VariantDef :: Union)) , VariantDef :: Variant (v) => Some (v . source (db) ? . map (ast :: VariantDef :: Variant)) , } } }
};
}
