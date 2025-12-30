// Generated macro for InvalidIssueString (struct)
macro_rules! Depcrate_session_diagnosticsInvalidIssueString {
() => {
// Module: crate::session_diagnostics
// Provides: {"InvalidIssueString"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (attr_parsing_invalid_issue_string , code = E0545)] pub (crate) struct InvalidIssueString { # [primary_span] pub span : Span , # [subdiagnostic] pub cause : Option < InvalidIssueStringCause > , }
};
}
