// Generated macro for AsDerefSuggestion (struct)
macro_rules! Depcrate_errorsAsDerefSuggestion {
() => {
// Module: crate::errors
// Provides: {"AsDerefSuggestion"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [suggestion (hir_typeck_as_deref_suggestion , code = ".as_deref()" , style = "verbose" , applicability = "maybe-incorrect")] pub (crate) struct AsDerefSuggestion { # [primary_span] pub (crate) span : Span , }
};
}
