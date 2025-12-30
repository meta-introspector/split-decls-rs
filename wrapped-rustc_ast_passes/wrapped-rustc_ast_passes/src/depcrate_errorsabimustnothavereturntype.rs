// Generated macro for AbiMustNotHaveReturnType (struct)
macro_rules! Depcrate_errorsAbiMustNotHaveReturnType {
() => {
// Module: crate::errors
// Provides: {"AbiMustNotHaveReturnType"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_passes_abi_must_not_have_return_type)] # [note] pub (crate) struct AbiMustNotHaveReturnType { # [primary_span] # [help] pub span : Span , pub abi : ExternAbi , }
};
}
