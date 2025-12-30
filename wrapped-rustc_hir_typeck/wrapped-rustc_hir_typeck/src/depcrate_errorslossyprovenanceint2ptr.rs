// Generated macro for LossyProvenanceInt2Ptr (struct)
macro_rules! Depcrate_errorsLossyProvenanceInt2Ptr {
() => {
// Module: crate::errors
// Provides: {"LossyProvenanceInt2Ptr"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (hir_typeck_lossy_provenance_int2ptr)] # [help] pub (crate) struct LossyProvenanceInt2Ptr < 'tcx > { pub expr_ty : Ty < 'tcx > , pub cast_ty : Ty < 'tcx > , # [subdiagnostic] pub sugg : LossyProvenanceInt2PtrSuggestion , }
};
}
