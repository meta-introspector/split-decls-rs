// Generated macro for impl_531 (impl)
macro_rules! Depcrateimpl_531 {
() => {
// Module: crate
// Provides: {"impl_531"}
// Dependencies: {}
impl HasContainer for TypeAlias { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { container_id_to_hir (self . id . lookup (db) . container) } }
};
}
