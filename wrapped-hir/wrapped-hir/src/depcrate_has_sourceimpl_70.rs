// Generated macro for impl_70 (impl)
macro_rules! Depcrate_has_sourceimpl_70 {
() => {
// Module: crate::has_source
// Provides: {"impl_70"}
// Dependencies: {}
impl HasSource for Function { type Ast = ast :: Fn ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { Some (self . id . lookup (db) . source (db)) } }
};
}
