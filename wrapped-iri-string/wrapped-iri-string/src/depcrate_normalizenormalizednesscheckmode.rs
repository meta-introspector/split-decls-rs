// Generated macro for NormalizednessCheckMode (enum)
macro_rules! Depcrate_normalizeNormalizednessCheckMode {
() => {
// Module: crate::normalize
// Provides: {"NormalizednessCheckMode"}
// Dependencies: {}
# [doc = " Normalizedness check algorithm."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub (crate) enum NormalizednessCheckMode { # [doc = " Default algorithm (corresponding to [`NormalizationMode::Default`])."] Default , # [doc = " Strict RFC 3986 normalization."] Rfc3986 , # [doc = " WHATWG-like normalization algorithm (corresponding to"] # [doc = " [`NormalizationMode::PreserveAuthoritylessRelativePath`])."] PreserveAuthoritylessRelativePath , }
};
}
