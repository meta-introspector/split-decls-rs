// Generated macro for impl_533 (impl)
macro_rules! Depcrateimpl_533 {
() => {
// Module: crate
// Provides: {"impl_533"}
// Dependencies: {}
impl HasContainer for Static { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { container_id_to_hir (self . id . lookup (db) . container) } }
};
}
