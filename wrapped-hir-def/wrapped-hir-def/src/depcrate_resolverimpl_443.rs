// Generated macro for impl_443 (impl)
macro_rules! Depcrate_resolverimpl_443 {
() => {
// Module: crate::resolver
// Provides: {"impl_443"}
// Dependencies: {}
impl HasResolver for ConstId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { lookup_resolver (db , self) } }
};
}
