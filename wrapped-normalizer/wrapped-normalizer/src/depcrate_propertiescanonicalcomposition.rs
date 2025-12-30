// Generated macro for CanonicalComposition (struct)
macro_rules! Depcrate_propertiesCanonicalComposition {
() => {
// Module: crate::properties
// Provides: {"CanonicalComposition"}
// Dependencies: {}
# [doc = " The raw canonical composition operation."] # [doc = ""] # [doc = " Callers should generally use `ComposingNormalizer` instead of this API."] # [doc = " However, this API is provided for callers such as HarfBuzz that specifically"] # [doc = " want access to the raw canonical composition operation e.g. for use in a"] # [doc = " glyph-availability-guided custom normalizer."] # [derive (Debug)] pub struct CanonicalComposition { canonical_compositions : DataPayload < NormalizerNfcV1 > , }
};
}
