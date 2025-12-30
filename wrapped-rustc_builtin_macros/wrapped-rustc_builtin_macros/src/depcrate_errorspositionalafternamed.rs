// Generated macro for PositionalAfterNamed (struct)
macro_rules! Depcrate_errorsPositionalAfterNamed {
() => {
// Module: crate::errors
// Provides: {"PositionalAfterNamed"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (builtin_macros_format_positional_after_named)] pub (crate) struct PositionalAfterNamed { # [primary_span] # [label] pub (crate) span : Span , # [label (builtin_macros_named_args)] pub (crate) args : Vec < Span > , }
};
}
