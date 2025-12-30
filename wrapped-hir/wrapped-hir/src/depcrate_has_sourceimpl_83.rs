// Generated macro for impl_83 (impl)
macro_rules! Depcrate_has_sourceimpl_83 {
() => {
// Module: crate::has_source
// Provides: {"impl_83"}
// Dependencies: {}
impl HasSource for ExternCrateDecl { type Ast = ast :: ExternCrate ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { Some (self . id . lookup (db) . source (db)) } }
};
}
