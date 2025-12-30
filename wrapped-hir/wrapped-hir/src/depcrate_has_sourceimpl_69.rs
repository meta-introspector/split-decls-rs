// Generated macro for impl_69 (impl)
macro_rules! Depcrate_has_sourceimpl_69 {
() => {
// Module: crate::has_source
// Provides: {"impl_69"}
// Dependencies: {}
impl HasSource for Variant { type Ast = ast :: Variant ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < ast :: Variant > > { Some (self . id . lookup (db) . source (db)) } }
};
}
