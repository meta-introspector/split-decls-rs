// Generated macro for CanonicalDecomposition (struct)
macro_rules! Depcrate_propertiesCanonicalDecomposition {
() => {
// Module: crate::properties
// Provides: {"CanonicalDecomposition"}
// Dependencies: {}
# [doc = " The raw (non-recursive) canonical decomposition operation."] # [doc = ""] # [doc = " Callers should generally use `DecomposingNormalizer` instead of this API."] # [doc = " However, this API is provided for callers such as HarfBuzz that specifically"] # [doc = " want access to non-recursive canonical decomposition e.g. for use in a"] # [doc = " glyph-availability-guided custom normalizer."] # [derive (Debug)] pub struct CanonicalDecomposition { decompositions : DataPayload < NormalizerNfdDataV1 > , tables : DataPayload < NormalizerNfdTablesV1 > , non_recursive : DataPayload < NormalizerNfdSupplementV1 > , }
};
}
