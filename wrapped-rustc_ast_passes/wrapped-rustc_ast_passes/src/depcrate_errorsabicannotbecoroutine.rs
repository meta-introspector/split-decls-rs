// Generated macro for AbiCannotBeCoroutine (struct)
macro_rules! Depcrate_errorsAbiCannotBeCoroutine {
() => {
// Module: crate::errors
// Provides: {"AbiCannotBeCoroutine"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_passes_abi_cannot_be_coroutine)] pub (crate) struct AbiCannotBeCoroutine { # [primary_span] pub span : Span , pub abi : ExternAbi , # [suggestion (ast_passes_suggestion , applicability = "maybe-incorrect" , code = "" , style = "verbose")] pub coroutine_kind_span : Span , pub coroutine_kind_str : & 'static str , }
};
}
