// Generated macro for LossyProvenancePtr2Int (struct)
macro_rules! Depcrate_errorsLossyProvenancePtr2Int {
() => {
// Module: crate::errors
// Provides: {"LossyProvenancePtr2Int"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (hir_typeck_lossy_provenance_ptr2int)] # [help] pub (crate) struct LossyProvenancePtr2Int < 'tcx > { pub expr_ty : Ty < 'tcx > , pub cast_ty : Ty < 'tcx > , # [subdiagnostic] pub sugg : LossyProvenancePtr2IntSuggestion < 'tcx > , }
};
}
