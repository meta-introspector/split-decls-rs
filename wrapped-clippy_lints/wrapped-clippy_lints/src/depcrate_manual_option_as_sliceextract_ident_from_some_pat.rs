// Generated macro for extract_ident_from_some_pat (function)
macro_rules! Depcrate_manual_option_as_sliceextract_ident_from_some_pat {
() => {
// Module: crate::manual_option_as_slice
// Provides: {"extract_ident_from_some_pat"}
// Dependencies: {}
fn extract_ident_from_some_pat (cx : & LateContext < '_ > , pat : & Pat < '_ >) -> Option < Symbol > { if let Some ([binding]) = as_some_pattern (cx , pat) && let PatKind :: Binding (_mode , _hir_id , ident , _inner_pat) = binding . kind { Some (ident . name) } else { None } }
};
}
