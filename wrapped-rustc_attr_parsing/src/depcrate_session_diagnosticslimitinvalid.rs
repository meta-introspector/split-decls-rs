// Generated macro for LimitInvalid (struct)
macro_rules! Depcrate_session_diagnosticsLimitInvalid {
() => {
// Module: crate::session_diagnostics
// Provides: {"LimitInvalid"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (attr_parsing_limit_invalid)] pub (crate) struct LimitInvalid < 'a > { # [primary_span] pub span : Span , # [label] pub value_span : Span , pub error_str : & 'a str , }
};
}
