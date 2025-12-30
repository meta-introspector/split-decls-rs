// Generated macro for impl_532 (impl)
macro_rules! Depcrateimpl_532 {
() => {
// Module: crate
// Provides: {"impl_532"}
// Dependencies: {}
impl HasContainer for Const { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { container_id_to_hir (self . id . lookup (db) . container) } }
};
}
