// Generated macro for impl_524 (impl)
macro_rules! Depcrateimpl_524 {
() => {
// Module: crate
// Provides: {"impl_524"}
// Dependencies: {}
impl HasContainer for ExternBlock { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { ItemContainer :: Module (Module { id : self . id . lookup (db) . container }) } }
};
}
