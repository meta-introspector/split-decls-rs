// Generated macro for impl_77 (impl)
macro_rules! Depcrate_has_sourceimpl_77 {
() => {
// Module: crate::has_source
// Provides: {"impl_77"}
// Dependencies: {}
impl HasSource for TypeOrConstParam { type Ast = Either < ast :: TypeOrConstParam , ast :: Trait > ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { let child_source = self . id . parent . child_source (db) ; child_source . map (| it | it . get (self . id . local_id) . cloned ()) . transpose () } }
};
}
