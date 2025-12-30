// Generated macro for has_no_fields (function)
macro_rules! Depcrate_empty_with_bracketshas_no_fields {
() => {
// Module: crate::empty_with_brackets
// Provides: {"has_no_fields"}
// Dependencies: {}
fn has_no_fields (cx : & LateContext < '_ > , var_data : & VariantData < '_ > , braces_span : Span) -> bool { var_data . fields () . is_empty () && ! span_contains_cfg (cx , braces_span) }
};
}
