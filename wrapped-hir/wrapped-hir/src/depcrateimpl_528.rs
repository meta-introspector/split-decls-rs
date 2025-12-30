// Generated macro for impl_528 (impl)
macro_rules! Depcrateimpl_528 {
() => {
// Module: crate
// Provides: {"impl_528"}
// Dependencies: {}
impl HasContainer for Struct { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { ItemContainer :: Module (Module { id : self . id . lookup (db) . container }) } }
};
}
