// Generated macro for IncompatibleFeatures (struct)
macro_rules! Depcrate_errorsIncompatibleFeatures {
() => {
// Module: crate::errors
// Provides: {"IncompatibleFeatures"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_passes_incompatible_features)] # [help] pub (crate) struct IncompatibleFeatures { # [primary_span] pub spans : Vec < Span > , pub f1 : Symbol , pub f2 : Symbol , }
};
}
