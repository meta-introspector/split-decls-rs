// Generated macro for FormatRedundantArgs (struct)
macro_rules! Depcrate_errorsFormatRedundantArgs {
() => {
// Module: crate::errors
// Provides: {"FormatRedundantArgs"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (builtin_macros_format_redundant_args)] pub (crate) struct FormatRedundantArgs { # [primary_span] pub (crate) span : MultiSpan , pub (crate) n : usize , # [note] pub (crate) note : MultiSpan , # [subdiagnostic] pub (crate) sugg : Option < FormatRedundantArgsSugg > , }
};
}
