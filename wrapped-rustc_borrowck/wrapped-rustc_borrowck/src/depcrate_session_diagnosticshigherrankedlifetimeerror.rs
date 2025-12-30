// Generated macro for HigherRankedLifetimeError (struct)
macro_rules! Depcrate_session_diagnosticsHigherRankedLifetimeError {
() => {
// Module: crate::session_diagnostics
// Provides: {"HigherRankedLifetimeError"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (borrowck_higher_ranked_lifetime_error)] pub (crate) struct HigherRankedLifetimeError { # [subdiagnostic] pub cause : Option < HigherRankedErrorCause > , # [primary_span] pub span : Span , }
};
}
