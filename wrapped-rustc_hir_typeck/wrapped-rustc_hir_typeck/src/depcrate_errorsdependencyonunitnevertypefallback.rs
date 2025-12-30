// Generated macro for DependencyOnUnitNeverTypeFallback (struct)
macro_rules! Depcrate_errorsDependencyOnUnitNeverTypeFallback {
() => {
// Module: crate::errors
// Provides: {"DependencyOnUnitNeverTypeFallback"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [help] # [diag (hir_typeck_dependency_on_unit_never_type_fallback)] pub (crate) struct DependencyOnUnitNeverTypeFallback < 'tcx > { # [note] pub obligation_span : Span , pub obligation : ty :: Predicate < 'tcx > , # [subdiagnostic] pub sugg : SuggestAnnotations , }
};
}
