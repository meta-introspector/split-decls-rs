// Generated macro for impl_530 (impl)
macro_rules! Depcrateimpl_530 {
() => {
// Module: crate
// Provides: {"impl_530"}
// Dependencies: {}
impl HasContainer for Enum { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { ItemContainer :: Module (Module { id : self . id . lookup (db) . container }) } }
};
}
