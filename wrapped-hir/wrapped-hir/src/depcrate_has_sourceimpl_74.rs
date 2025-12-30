// Generated macro for impl_74 (impl)
macro_rules! Depcrate_has_sourceimpl_74 {
() => {
// Module: crate::has_source
// Provides: {"impl_74"}
// Dependencies: {}
impl HasSource for TypeAlias { type Ast = ast :: TypeAlias ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { Some (self . id . lookup (db) . source (db)) } }
};
}
