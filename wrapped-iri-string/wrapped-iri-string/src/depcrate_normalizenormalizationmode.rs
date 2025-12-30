// Generated macro for NormalizationMode (enum)
macro_rules! Depcrate_normalizeNormalizationMode {
() => {
// Module: crate::normalize
// Provides: {"NormalizationMode"}
// Dependencies: {}
# [doc = " Normalization algorithm."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub (crate) enum NormalizationMode { # [doc = " No normalization."] None , # [doc = " Default normalization mode."] # [doc = ""] # [doc = " Applies RFC 3986 normalization whenever possible. When not possible,"] # [doc = " applies serialization algorithm defined in WHATWG URL standard."] Default , # [doc = " WHATWG-like normalization mode."] # [doc = ""] # [doc = " Preserves relative path as is (modulo case/pct normalization) when the"] # [doc = " authority component is absent."] PreserveAuthoritylessRelativePath , }
};
}
