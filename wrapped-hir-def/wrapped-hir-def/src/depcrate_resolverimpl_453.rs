// Generated macro for impl_453 (impl)
macro_rules! Depcrate_resolverimpl_453 {
() => {
// Module: crate::resolver
// Provides: {"impl_453"}
// Dependencies: {}
impl HasResolver for EnumVariantId { fn resolver (self , db : & dyn DefDatabase) -> Resolver < '_ > { self . lookup (db) . parent . resolver (db) } }
};
}
