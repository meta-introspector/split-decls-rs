// Generated macro for impl_73 (impl)
macro_rules! Depcrate_has_sourceimpl_73 {
() => {
// Module: crate::has_source
// Provides: {"impl_73"}
// Dependencies: {}
impl HasSource for Trait { type Ast = ast :: Trait ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { Some (self . id . lookup (db) . source (db)) } }
};
}
