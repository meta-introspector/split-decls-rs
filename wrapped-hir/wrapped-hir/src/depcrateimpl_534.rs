// Generated macro for impl_534 (impl)
macro_rules! Depcrateimpl_534 {
() => {
// Module: crate
// Provides: {"impl_534"}
// Dependencies: {}
impl HasContainer for Trait { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { ItemContainer :: Module (Module { id : self . id . lookup (db) . container }) } }
};
}
