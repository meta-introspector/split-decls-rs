// Generated macro for InvalidPredicate (struct)
macro_rules! Depcrate_session_diagnosticsInvalidPredicate {
() => {
// Module: crate::session_diagnostics
// Provides: {"InvalidPredicate"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (attr_parsing_invalid_predicate , code = E0537)] pub (crate) struct InvalidPredicate { # [primary_span] pub span : Span , pub predicate : String , }
};
}
