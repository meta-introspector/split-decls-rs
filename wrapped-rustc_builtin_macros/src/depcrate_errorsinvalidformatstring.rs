// Generated macro for InvalidFormatString (struct)
macro_rules! Depcrate_errorsInvalidFormatString {
() => {
// Module: crate::errors
// Provides: {"InvalidFormatString"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (builtin_macros_format_string_invalid)] pub (crate) struct InvalidFormatString { # [primary_span] # [label] pub (crate) span : Span , pub (crate) desc : String , pub (crate) label1 : String , # [subdiagnostic] pub (crate) note_ : Option < InvalidFormatStringNote > , # [subdiagnostic] pub (crate) label_ : Option < InvalidFormatStringLabel > , # [subdiagnostic] pub (crate) sugg_ : Option < InvalidFormatStringSuggestion > , }
};
}
