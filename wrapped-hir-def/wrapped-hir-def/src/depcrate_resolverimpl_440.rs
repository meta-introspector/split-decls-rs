// Generated macro for impl_440 (impl)
macro_rules! Depcrate_resolverimpl_440 {
() => {
// Module: crate::resolver
// Provides: {"impl_440"}
// Dependencies: {}
impl HasResolver for TraitId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { lookup_resolver (db , self) . push_generic_params_scope (db , self . into ()) } }
};
}
