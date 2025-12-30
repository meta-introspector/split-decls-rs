// Generated macro for impl_bool_lit_to_pm_lit (macro)
macro_rules! Depcrate_implsimpl_bool_lit_to_pm_lit {
() => {
// Module: crate::impls
// Provides: {"impl_bool_lit_to_pm_lit"}
// Dependencies: {}
macro_rules ! impl_bool_lit_to_pm_lit { ([$ ($ prefix : tt) *] =>) => { impl From < crate :: BoolLit > for $ ($ prefix) * Ident { fn from (l : crate :: BoolLit) -> Self { Self :: new (l . as_str () , $ ($ prefix) * Span :: call_site ()) } } } ; }
};
}
