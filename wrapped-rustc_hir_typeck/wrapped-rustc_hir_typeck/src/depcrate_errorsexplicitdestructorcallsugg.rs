// Generated macro for ExplicitDestructorCallSugg (enum)
macro_rules! Depcrate_errorsExplicitDestructorCallSugg {
() => {
// Module: crate::errors
// Provides: {"ExplicitDestructorCallSugg"}
// Dependencies: {}
# [derive (Subdiagnostic)] pub (crate) enum ExplicitDestructorCallSugg { # [suggestion (hir_typeck_suggestion , code = "drop" , applicability = "maybe-incorrect")] Empty (# [primary_span] Span) , # [multipart_suggestion (hir_typeck_suggestion , style = "short")] Snippet { # [suggestion_part (code = "drop(")] lo : Span , # [suggestion_part (code = ")")] hi : Span , } , }
};
}
