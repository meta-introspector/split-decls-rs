// Generated macro for CoroutineAndCVariadic (struct)
macro_rules! Depcrate_errorsCoroutineAndCVariadic {
() => {
// Module: crate::errors
// Provides: {"CoroutineAndCVariadic"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_passes_coroutine_and_c_variadic)] pub (crate) struct CoroutineAndCVariadic { # [primary_span] pub spans : Vec < Span > , pub coroutine_kind : & 'static str , # [label (ast_passes_const)] pub coroutine_span : Span , # [label (ast_passes_variadic)] pub variadic_span : Span , }
};
}
