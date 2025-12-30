// Generated macro for AbiSpecifiedMultipleTimes (struct)
macro_rules! Depcrate_errorsAbiSpecifiedMultipleTimes {
() => {
// Module: crate::errors
// Provides: {"AbiSpecifiedMultipleTimes"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_lowering_abi_specified_multiple_times)] pub (crate) struct AbiSpecifiedMultipleTimes { # [primary_span] pub abi_span : Span , pub prev_name : Symbol , # [label] pub prev_span : Span , # [note] pub equivalent : bool , }
};
}
