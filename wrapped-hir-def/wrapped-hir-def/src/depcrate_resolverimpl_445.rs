// Generated macro for impl_445 (impl)
macro_rules! Depcrate_resolverimpl_445 {
() => {
// Module: crate::resolver
// Provides: {"impl_445"}
// Dependencies: {}
impl HasResolver for TypeAliasId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { lookup_resolver (db , self) . push_generic_params_scope (db , self . into ()) } }
};
}
