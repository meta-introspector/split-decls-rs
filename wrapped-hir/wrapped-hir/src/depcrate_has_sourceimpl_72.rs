// Generated macro for impl_72 (impl)
macro_rules! Depcrate_has_sourceimpl_72 {
() => {
// Module: crate::has_source
// Provides: {"impl_72"}
// Dependencies: {}
impl HasSource for Static { type Ast = ast :: Static ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { Some (self . id . lookup (db) . source (db)) } }
};
}
