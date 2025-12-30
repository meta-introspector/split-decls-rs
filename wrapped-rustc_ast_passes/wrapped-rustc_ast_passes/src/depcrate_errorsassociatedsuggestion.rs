// Generated macro for AssociatedSuggestion (struct)
macro_rules! Depcrate_errorsAssociatedSuggestion {
() => {
// Module: crate::errors
// Provides: {"AssociatedSuggestion"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [suggestion (ast_passes_suggestion , code = "{param}: {path}" , style = "verbose" , applicability = "maybe-incorrect")] pub (crate) struct AssociatedSuggestion { # [primary_span] pub span : Span , pub ident : Ident , pub param : Ident , pub path : String , }
};
}
