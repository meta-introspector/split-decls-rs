// Generated macro for impl_450 (impl)
macro_rules! Depcrate_resolverimpl_450 {
() => {
// Module: crate::resolver
// Provides: {"impl_450"}
// Dependencies: {}
impl HasResolver for DefWithBodyId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { match self { DefWithBodyId :: ConstId (c) => c . resolver (db) , DefWithBodyId :: FunctionId (f) => f . resolver (db) , DefWithBodyId :: StaticId (s) => s . resolver (db) , DefWithBodyId :: VariantId (v) => v . resolver (db) , } } }
};
}
