// Generated macro for impl_76 (impl)
macro_rules! Depcrate_has_sourceimpl_76 {
() => {
// Module: crate::has_source
// Provides: {"impl_76"}
// Dependencies: {}
impl HasSource for Impl { type Ast = ast :: Impl ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { Some (self . id . lookup (db) . source (db)) } }
};
}
