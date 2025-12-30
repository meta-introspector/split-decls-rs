// Generated macro for impl_66 (impl)
macro_rules! Depcrate_has_sourceimpl_66 {
() => {
// Module: crate::has_source
// Provides: {"impl_66"}
// Dependencies: {}
impl HasSource for Struct { type Ast = ast :: Struct ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { Some (self . id . lookup (db) . source (db)) } }
};
}
