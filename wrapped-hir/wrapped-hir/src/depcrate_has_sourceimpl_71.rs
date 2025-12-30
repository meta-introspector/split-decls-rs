// Generated macro for impl_71 (impl)
macro_rules! Depcrate_has_sourceimpl_71 {
() => {
// Module: crate::has_source
// Provides: {"impl_71"}
// Dependencies: {}
impl HasSource for Const { type Ast = ast :: Const ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { Some (self . id . lookup (db) . source (db)) } }
};
}
