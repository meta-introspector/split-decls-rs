// Generated macro for AbiCannotBeCalled (struct)
macro_rules! Depcrate_errorsAbiCannotBeCalled {
() => {
// Module: crate::errors
// Provides: {"AbiCannotBeCalled"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_typeck_abi_cannot_be_called)] pub (crate) struct AbiCannotBeCalled { # [primary_span] # [note] pub span : Span , pub abi : ExternAbi , }
};
}
