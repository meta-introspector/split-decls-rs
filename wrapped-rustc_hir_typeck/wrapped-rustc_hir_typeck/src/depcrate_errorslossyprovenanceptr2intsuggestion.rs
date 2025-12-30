// Generated macro for LossyProvenancePtr2IntSuggestion (enum)
macro_rules! Depcrate_errorsLossyProvenancePtr2IntSuggestion {
() => {
// Module: crate::errors
// Provides: {"LossyProvenancePtr2IntSuggestion"}
// Dependencies: {}
# [derive (Subdiagnostic)] pub (crate) enum LossyProvenancePtr2IntSuggestion < 'tcx > { # [multipart_suggestion (hir_typeck_suggestion , applicability = "maybe-incorrect")] NeedsParensCast { # [suggestion_part (code = "(")] expr_span : Span , # [suggestion_part (code = ").addr() as {cast_ty}")] cast_span : Span , cast_ty : Ty < 'tcx > , } , # [multipart_suggestion (hir_typeck_suggestion , applicability = "maybe-incorrect")] NeedsParens { # [suggestion_part (code = "(")] expr_span : Span , # [suggestion_part (code = ").addr()")] cast_span : Span , } , # [suggestion (hir_typeck_suggestion , code = ".addr() as {cast_ty}" , applicability = "maybe-incorrect")] NeedsCast { # [primary_span] cast_span : Span , cast_ty : Ty < 'tcx > , } , # [suggestion (hir_typeck_suggestion , code = ".addr()" , applicability = "maybe-incorrect")] Other { # [primary_span] cast_span : Span , } , }
};
}
