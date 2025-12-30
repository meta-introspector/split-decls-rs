// Generated macro for FormatPositionalMismatch (struct)
macro_rules! Depcrate_errorsFormatPositionalMismatch {
() => {
// Module: crate::errors
// Provides: {"FormatPositionalMismatch"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (builtin_macros_format_pos_mismatch)] pub (crate) struct FormatPositionalMismatch { # [primary_span] pub (crate) span : MultiSpan , pub (crate) n : usize , pub (crate) desc : String , # [subdiagnostic] pub (crate) highlight : SingleLabelManySpans , }
};
}
