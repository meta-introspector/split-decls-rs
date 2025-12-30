// Generated macro for impl_447 (impl)
macro_rules! Depcrate_resolverimpl_447 {
() => {
// Module: crate::resolver
// Provides: {"impl_447"}
// Dependencies: {}
impl HasResolver for ExternBlockId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { lookup_resolver (db , self) } }
};
}
