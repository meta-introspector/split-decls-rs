// Generated macro for impl_446 (impl)
macro_rules! Depcrate_resolverimpl_446 {
() => {
// Module: crate::resolver
// Provides: {"impl_446"}
// Dependencies: {}
impl HasResolver for ImplId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { self . lookup (db) . container . resolver (db) . push_generic_params_scope (db , self . into ()) } }
};
}
