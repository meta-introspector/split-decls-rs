// Generated macro for FailedToGetLayout (struct)
macro_rules! Depcrate_errorsFailedToGetLayout {
() => {
// Module: crate::errors
// Provides: {"FailedToGetLayout"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (codegen_ssa_failed_to_get_layout)] pub struct FailedToGetLayout < 'tcx > { # [primary_span] pub span : Span , pub ty : Ty < 'tcx > , pub err : LayoutError < 'tcx > , }
};
}
