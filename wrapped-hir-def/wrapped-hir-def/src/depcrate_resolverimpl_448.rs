// Generated macro for impl_448 (impl)
macro_rules! Depcrate_resolverimpl_448 {
() => {
// Module: crate::resolver
// Provides: {"impl_448"}
// Dependencies: {}
impl HasResolver for ExternCrateId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { lookup_resolver (db , self) } }
};
}
