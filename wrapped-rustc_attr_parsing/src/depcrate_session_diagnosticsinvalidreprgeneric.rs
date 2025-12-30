// Generated macro for InvalidReprGeneric (struct)
macro_rules! Depcrate_session_diagnosticsInvalidReprGeneric {
() => {
// Module: crate::session_diagnostics
// Provides: {"InvalidReprGeneric"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (attr_parsing_invalid_repr_generic , code = E0589)] pub (crate) struct InvalidReprGeneric < 'a > { # [primary_span] pub span : Span , pub repr_arg : String , pub error_part : & 'a str , }
};
}
