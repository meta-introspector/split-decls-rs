// Generated macro for impl_442 (impl)
macro_rules! Depcrate_resolverimpl_442 {
() => {
// Module: crate::resolver
// Provides: {"impl_442"}
// Dependencies: {}
impl HasResolver for FunctionId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { lookup_resolver (db , self) . push_generic_params_scope (db , self . into ()) } }
};
}
