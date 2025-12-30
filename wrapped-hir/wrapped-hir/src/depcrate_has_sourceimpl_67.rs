// Generated macro for impl_67 (impl)
macro_rules! Depcrate_has_sourceimpl_67 {
() => {
// Module: crate::has_source
// Provides: {"impl_67"}
// Dependencies: {}
impl HasSource for Union { type Ast = ast :: Union ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { Some (self . id . lookup (db) . source (db)) } }
};
}
