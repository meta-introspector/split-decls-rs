// Generated macro for CannotCastToBool (struct)
macro_rules! Depcrate_errorsCannotCastToBool {
() => {
// Module: crate::errors
// Provides: {"CannotCastToBool"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_typeck_cannot_cast_to_bool , code = E0054)] pub (crate) struct CannotCastToBool < 'tcx > { # [primary_span] pub span : Span , pub expr_ty : Ty < 'tcx > , # [subdiagnostic] pub help : CannotCastToBoolHelp , }
};
}
