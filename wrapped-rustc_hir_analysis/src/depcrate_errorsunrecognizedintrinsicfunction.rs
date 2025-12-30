// Generated macro for UnrecognizedIntrinsicFunction (struct)
macro_rules! Depcrate_errorsUnrecognizedIntrinsicFunction {
() => {
// Module: crate::errors
// Provides: {"UnrecognizedIntrinsicFunction"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_analysis_unrecognized_intrinsic_function , code = E0093)] # [help] pub (crate) struct UnrecognizedIntrinsicFunction { # [primary_span] # [label] pub span : Span , pub name : Symbol , }
};
}
