// Generated macro for AssociatedSuggestion2 (struct)
macro_rules! Depcrate_errorsAssociatedSuggestion2 {
() => {
// Module: crate::errors
// Provides: {"AssociatedSuggestion2"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [multipart_suggestion (ast_passes_suggestion_path , applicability = "maybe-incorrect")] pub (crate) struct AssociatedSuggestion2 { # [suggestion_part (code = "{args}")] pub span : Span , pub args : String , # [suggestion_part (code = "")] pub predicate : Span , pub trait_segment : Ident , pub potential_assoc : Ident , }
};
}
