// Generated macro for ConcatBytesInvalid (struct)
macro_rules! Depcrate_errorsConcatBytesInvalid {
() => {
// Module: crate::errors
// Provides: {"ConcatBytesInvalid"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (builtin_macros_concat_bytes_invalid)] pub (crate) struct ConcatBytesInvalid { # [primary_span] pub (crate) span : Span , pub (crate) lit_kind : & 'static str , # [subdiagnostic] pub (crate) sugg : Option < ConcatBytesInvalidSuggestion > , # [note (builtin_macros_c_str_note)] pub (crate) cs_note : Option < () > , }
};
}
