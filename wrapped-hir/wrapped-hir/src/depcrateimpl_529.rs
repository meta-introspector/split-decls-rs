// Generated macro for impl_529 (impl)
macro_rules! Depcrateimpl_529 {
() => {
// Module: crate
// Provides: {"impl_529"}
// Dependencies: {}
impl HasContainer for Union { fn container (& self , db : & dyn HirDatabase) -> ItemContainer { ItemContainer :: Module (Module { id : self . id . lookup (db) . container }) } }
};
}
