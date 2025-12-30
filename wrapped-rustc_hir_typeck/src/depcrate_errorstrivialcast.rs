// Generated macro for TrivialCast (struct)
macro_rules! Depcrate_errorsTrivialCast {
() => {
// Module: crate::errors
// Provides: {"TrivialCast"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (hir_typeck_trivial_cast)] # [help] pub (crate) struct TrivialCast < 'tcx > { pub numeric : bool , pub expr_ty : Ty < 'tcx > , pub cast_ty : Ty < 'tcx > , }
};
}
