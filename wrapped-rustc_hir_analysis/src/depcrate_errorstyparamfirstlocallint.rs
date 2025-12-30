// Generated macro for TyParamFirstLocalLint (struct)
macro_rules! Depcrate_errorsTyParamFirstLocalLint {
() => {
// Module: crate::errors
// Provides: {"TyParamFirstLocalLint"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (hir_analysis_ty_param_first_local , code = E0210)] # [note] pub (crate) struct TyParamFirstLocalLint < 'tcx > { # [label] pub span : Span , # [note (hir_analysis_case_note)] pub note : () , pub param : Ident , pub local_type : Ty < 'tcx > , }
};
}
