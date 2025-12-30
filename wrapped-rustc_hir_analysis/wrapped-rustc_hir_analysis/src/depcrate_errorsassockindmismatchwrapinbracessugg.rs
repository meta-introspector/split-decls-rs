// Generated macro for AssocKindMismatchWrapInBracesSugg (struct)
macro_rules! Depcrate_errorsAssocKindMismatchWrapInBracesSugg {
() => {
// Module: crate::errors
// Provides: {"AssocKindMismatchWrapInBracesSugg"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [multipart_suggestion (hir_analysis_assoc_kind_mismatch_wrap_in_braces_sugg , applicability = "maybe-incorrect")] pub (crate) struct AssocKindMismatchWrapInBracesSugg { # [suggestion_part (code = "{{ ")] pub lo : Span , # [suggestion_part (code = " }}")] pub hi : Span , }
};
}
