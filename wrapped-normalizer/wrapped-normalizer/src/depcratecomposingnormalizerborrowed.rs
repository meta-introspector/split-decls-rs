// Generated macro for ComposingNormalizerBorrowed (struct)
macro_rules! DepcrateComposingNormalizerBorrowed {
() => {
// Module: crate
// Provides: {"ComposingNormalizerBorrowed"}
// Dependencies: {}
# [doc = " Borrowed version of a normalizer for performing composing normalization."] # [derive (Debug)] pub struct ComposingNormalizerBorrowed < 'a > { decomposing_normalizer : DecomposingNormalizerBorrowed < 'a > , canonical_compositions : & 'a CanonicalCompositions < 'a > , }
};
}
