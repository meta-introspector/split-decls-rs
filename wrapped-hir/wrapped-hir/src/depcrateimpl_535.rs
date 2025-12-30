// Generated macro for impl_535 (impl)
macro_rules! Depcrateimpl_535 {
() => {
// Module: crate
// Provides: {"impl_535"}
// Dependencies: {}
impl HasContainer for ExternBlock { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { ItemContainer :: Module (Module { id : self . id . lookup (db) . container }) } }
};
}
