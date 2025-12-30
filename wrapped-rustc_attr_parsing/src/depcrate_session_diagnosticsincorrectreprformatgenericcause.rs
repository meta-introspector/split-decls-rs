// Generated macro for IncorrectReprFormatGenericCause (enum)
macro_rules! Depcrate_session_diagnosticsIncorrectReprFormatGenericCause {
() => {
// Module: crate::session_diagnostics
// Provides: {"IncorrectReprFormatGenericCause"}
// Dependencies: {}
# [derive (Subdiagnostic)] pub (crate) enum IncorrectReprFormatGenericCause { # [suggestion (attr_parsing_suggestion , code = "{name}({value})" , applicability = "machine-applicable")] Int { # [primary_span] span : Span , # [skip_arg] name : Symbol , # [skip_arg] value : u128 , } , # [suggestion (attr_parsing_suggestion , code = "{name}({value})" , applicability = "machine-applicable")] Symbol { # [primary_span] span : Span , # [skip_arg] name : Symbol , # [skip_arg] value : Symbol , } , }
};
}
