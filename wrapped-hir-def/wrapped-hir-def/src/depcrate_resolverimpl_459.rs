// Generated macro for impl_459 (impl)
macro_rules! Depcrate_resolverimpl_459 {
() => {
// Module: crate::resolver
// Provides: {"impl_459"}
// Dependencies: {}
impl HasResolver for EnumVariantId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { self . lookup (db) . parent . resolver (db) } }
};
}
