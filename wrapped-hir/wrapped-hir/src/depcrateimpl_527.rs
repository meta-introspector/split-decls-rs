// Generated macro for impl_527 (impl)
macro_rules! Depcrateimpl_527 {
() => {
// Module: crate
// Provides: {"impl_527"}
// Dependencies: {}
impl HasContainer for Function { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { container_id_to_hir (self . id . lookup (db) . container) } }
};
}
