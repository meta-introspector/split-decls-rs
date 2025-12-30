// Generated macro for impl_trait_query (function)
macro_rules! Depcrate_lowerimpl_trait_query {
() => {
// Module: crate::lower
// Provides: {"impl_trait_query"}
// Dependencies: {}
pub (crate) fn impl_trait_query < 'db > (db : & 'db dyn HirDatabase , impl_id : ImplId ,) -> Option < EarlyBinder < 'db , TraitRef < 'db > > > { db . impl_trait_with_diagnostics (impl_id) . map (| it | it . 0) }
};
}
