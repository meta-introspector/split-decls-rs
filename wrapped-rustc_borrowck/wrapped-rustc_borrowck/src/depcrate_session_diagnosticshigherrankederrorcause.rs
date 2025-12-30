// Generated macro for HigherRankedErrorCause (enum)
macro_rules! Depcrate_session_diagnosticsHigherRankedErrorCause {
() => {
// Module: crate::session_diagnostics
// Provides: {"HigherRankedErrorCause"}
// Dependencies: {}
# [derive (Subdiagnostic)] pub (crate) enum HigherRankedErrorCause { # [note (borrowck_could_not_prove)] CouldNotProve { predicate : String } , # [note (borrowck_could_not_normalize)] CouldNotNormalize { value : String } , }
};
}
