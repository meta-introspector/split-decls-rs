// Generated macro for ConstAndCoroutine (struct)
macro_rules! Depcrate_errorsConstAndCoroutine {
() => {
// Module: crate::errors
// Provides: {"ConstAndCoroutine"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_passes_const_and_coroutine)] pub (crate) struct ConstAndCoroutine { # [primary_span] pub spans : Vec < Span > , # [label (ast_passes_const)] pub const_span : Span , # [label (ast_passes_coroutine)] pub coroutine_span : Span , # [label] pub span : Span , pub coroutine_kind : & 'static str , }
};
}
