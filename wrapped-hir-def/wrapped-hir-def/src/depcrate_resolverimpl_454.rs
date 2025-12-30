// Generated macro for impl_454 (impl)
macro_rules! Depcrate_resolverimpl_454 {
() => {
// Module: crate::resolver
// Provides: {"impl_454"}
// Dependencies: {}
impl HasResolver for VariantId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { match self { VariantId :: EnumVariantId (it) => it . resolver (db) , VariantId :: StructId (it) => it . resolver (db) , VariantId :: UnionId (it) => it . resolver (db) , } } }
};
}
