// Generated macro for ConstAndCVariadic (struct)
macro_rules! Depcrate_errorsConstAndCVariadic {
() => {
// Module: crate::errors
// Provides: {"ConstAndCVariadic"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_passes_const_and_c_variadic)] pub (crate) struct ConstAndCVariadic { # [primary_span] pub spans : Vec < Span > , # [label (ast_passes_const)] pub const_span : Span , # [label (ast_passes_variadic)] pub variadic_span : Span , }
};
}
