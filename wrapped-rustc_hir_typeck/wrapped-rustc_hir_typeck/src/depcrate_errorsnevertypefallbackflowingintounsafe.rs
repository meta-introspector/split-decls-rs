// Generated macro for NeverTypeFallbackFlowingIntoUnsafe (enum)
macro_rules! Depcrate_errorsNeverTypeFallbackFlowingIntoUnsafe {
() => {
// Module: crate::errors
// Provides: {"NeverTypeFallbackFlowingIntoUnsafe"}
// Dependencies: {}
# [derive (LintDiagnostic)] pub (crate) enum NeverTypeFallbackFlowingIntoUnsafe { # [help] # [diag (hir_typeck_never_type_fallback_flowing_into_unsafe_call)] Call { # [subdiagnostic] sugg : SuggestAnnotations , } , # [help] # [diag (hir_typeck_never_type_fallback_flowing_into_unsafe_method)] Method { # [subdiagnostic] sugg : SuggestAnnotations , } , # [help] # [diag (hir_typeck_never_type_fallback_flowing_into_unsafe_path)] Path { # [subdiagnostic] sugg : SuggestAnnotations , } , # [help] # [diag (hir_typeck_never_type_fallback_flowing_into_unsafe_union_field)] UnionField { # [subdiagnostic] sugg : SuggestAnnotations , } , # [help] # [diag (hir_typeck_never_type_fallback_flowing_into_unsafe_deref)] Deref { # [subdiagnostic] sugg : SuggestAnnotations , } , }
};
}
