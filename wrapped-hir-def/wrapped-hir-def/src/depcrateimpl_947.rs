// Generated macro for impl_947 (impl)
macro_rules! Depcrateimpl_947 {
() => {
// Module: crate
// Provides: {"impl_947"}
// Dependencies: {}
impl HasModule for DefWithBodyId { fn module (& self , db : & dyn DefDatabase) -> ModuleId { match self { DefWithBodyId :: FunctionId (it) => it . module (db) , DefWithBodyId :: StaticId (it) => it . module (db) , DefWithBodyId :: ConstId (it) => it . module (db) , DefWithBodyId :: VariantId (it) => it . module (db) , } } }
};
}
