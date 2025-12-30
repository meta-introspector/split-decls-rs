// Generated macro for FnMutError (struct)
macro_rules! Depcrate_session_diagnosticsFnMutError {
() => {
// Module: crate::session_diagnostics
// Provides: {"FnMutError"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (borrowck_var_cannot_escape_closure)] # [note] # [note (borrowck_cannot_escape)] pub (crate) struct FnMutError { # [primary_span] pub span : Span , # [subdiagnostic] pub ty_err : FnMutReturnTypeErr , }
};
}
