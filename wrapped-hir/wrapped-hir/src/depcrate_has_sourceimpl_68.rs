// Generated macro for impl_68 (impl)
macro_rules! Depcrate_has_sourceimpl_68 {
() => {
// Module: crate::has_source
// Provides: {"impl_68"}
// Dependencies: {}
impl HasSource for Enum { type Ast = ast :: Enum ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { Some (self . id . lookup (db) . source (db)) } }
};
}
