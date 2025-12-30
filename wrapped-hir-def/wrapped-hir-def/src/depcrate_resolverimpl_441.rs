// Generated macro for impl_441 (impl)
macro_rules! Depcrate_resolverimpl_441 {
() => {
// Module: crate::resolver
// Provides: {"impl_441"}
// Dependencies: {}
impl < T : Into < AdtId > + Copy > HasResolver for T { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { let def = self . into () ; def . module (db) . resolver (db) . push_generic_params_scope (db , def . into ()) } }
};
}
