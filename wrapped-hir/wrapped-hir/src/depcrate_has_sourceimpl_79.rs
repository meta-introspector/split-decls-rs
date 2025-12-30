// Generated macro for impl_79 (impl)
macro_rules! Depcrate_has_sourceimpl_79 {
() => {
// Module: crate::has_source
// Provides: {"impl_79"}
// Dependencies: {}
impl HasSource for LocalSource { type Ast = Either < ast :: IdentPat , ast :: SelfParam > ; fn source (self , _ : & dyn HirDatabase) -> Option < InFile < Self :: Ast > > { Some (self . source) } }
};
}
