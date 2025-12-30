// Generated macro for LossyProvenanceInt2PtrSuggestion (struct)
macro_rules! Depcrate_errorsLossyProvenanceInt2PtrSuggestion {
() => {
// Module: crate::errors
// Provides: {"LossyProvenanceInt2PtrSuggestion"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [multipart_suggestion (hir_typeck_suggestion , applicability = "has-placeholders")] pub (crate) struct LossyProvenanceInt2PtrSuggestion { # [suggestion_part (code = "(...).with_addr(")] pub lo : Span , # [suggestion_part (code = ")")] pub hi : Span , }
};
}
