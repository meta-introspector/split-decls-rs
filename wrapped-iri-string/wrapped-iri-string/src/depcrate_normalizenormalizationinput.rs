// Generated macro for NormalizationInput (struct)
macro_rules! Depcrate_normalizeNormalizationInput {
() => {
// Module: crate::normalize
// Provides: {"NormalizationInput"}
// Dependencies: {}
# [doc = " Spec-agnostic IRI normalization/resolution input."] # [derive (Debug , Clone , Copy)] pub (crate) struct NormalizationInput < 'a > { # [doc = " Target scheme."] scheme : & 'a str , # [doc = " Target authority."] authority : Option < & 'a str > , # [doc = " Target path without dot-removal."] path : Path < 'a > , # [doc = " Target query."] query : Option < & 'a str > , # [doc = " Target fragment."] fragment : Option < & 'a str > , # [doc = " Normalization type."] op : NormalizationOp , }
};
}
