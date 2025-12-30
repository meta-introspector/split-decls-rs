// Generated macro for TyParamFirstLocal (struct)
macro_rules! Depcrate_errorsTyParamFirstLocal {
() => {
// Module: crate::errors
// Provides: {"TyParamFirstLocal"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_analysis_ty_param_first_local , code = E0210)] # [note] pub (crate) struct TyParamFirstLocal < 'tcx > { # [primary_span] # [label] pub span : Span , # [note (hir_analysis_case_note)] pub note : () , pub param : Ident , pub local_type : Ty < 'tcx > , }
};
}
