// Generated macro for impl_525 (impl)
macro_rules! Depcrateimpl_525 {
() => {
// Module: crate
// Provides: {"impl_525"}
// Dependencies: {}
impl HasContainer for ExternCrateDecl { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { container_id_to_hir (self . id . lookup (db) . container . into ()) } }
};
}
