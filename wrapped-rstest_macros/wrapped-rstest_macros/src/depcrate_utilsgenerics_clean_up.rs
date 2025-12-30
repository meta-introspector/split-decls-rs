// Generated macro for generics_clean_up (function)
macro_rules! Depcrate_utilsgenerics_clean_up {
() => {
// Module: crate::utils
// Provides: {"generics_clean_up"}
// Dependencies: {}
pub (crate) fn generics_clean_up < 'a > (original : & Generics , inputs : impl Iterator < Item = & 'a FnArg > , output : & ReturnType ,) -> syn :: Generics { let used = references_ident_types (original , inputs , output) ; let mut result : Generics = original . clone () ; result . params = filtered_generics (result . params . into_iter () , & used) . collect () ; result . where_clause = result . where_clause . map (| wc | filtered_predicates (wc , & used)) ; result }
};
}
