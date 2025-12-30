// Generated macro for NoFieldOnType (struct)
macro_rules! Depcrate_errorsNoFieldOnType {
() => {
// Module: crate::errors
// Provides: {"NoFieldOnType"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_typeck_no_field_on_type , code = E0609)] pub (crate) struct NoFieldOnType < 'tcx > { # [primary_span] pub (crate) span : Span , pub (crate) ty : Ty < 'tcx > , pub (crate) field : Ident , }
};
}
