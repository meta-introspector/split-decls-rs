// Generated macro for InvalidUnionFieldSuggestion (struct)
macro_rules! Depcrate_errorsInvalidUnionFieldSuggestion {
() => {
// Module: crate::errors
// Provides: {"InvalidUnionFieldSuggestion"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [multipart_suggestion (hir_analysis_invalid_union_field_sugg , applicability = "machine-applicable")] pub (crate) struct InvalidUnionFieldSuggestion { # [suggestion_part (code = "std::mem::ManuallyDrop<")] pub lo : Span , # [suggestion_part (code = ">")] pub hi : Span , }
};
}
