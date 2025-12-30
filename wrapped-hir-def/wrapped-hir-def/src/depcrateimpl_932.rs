// Generated macro for impl_932 (impl)
macro_rules! Depcrateimpl_932 {
() => {
// Module: crate
// Provides: {"impl_932"}
// Dependencies: {}
impl HasModule for EnumVariantId { # [inline] fn module (& self , db : & dyn DefDatabase) -> ModuleId { self . lookup (db) . parent . module (db) } }
};
}
