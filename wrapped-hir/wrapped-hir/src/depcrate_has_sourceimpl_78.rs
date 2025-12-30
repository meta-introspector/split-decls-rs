// Generated macro for impl_78 (impl)
macro_rules! Depcrate_has_sourceimpl_78 {
() => {
// Module: crate::has_source
// Provides: {"impl_78"}
// Dependencies: {}
impl HasSource for LifetimeParam { type Ast = ast :: LifetimeParam ; fn source (self , db : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { let child_source = self . id . parent . child_source (db) ; child_source . map (| it | it . get (self . id . local_id) . cloned ()) . transpose () } }
};
}
