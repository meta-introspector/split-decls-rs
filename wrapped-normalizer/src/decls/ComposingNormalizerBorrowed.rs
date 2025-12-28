macro_rules! deps {
    () => {
        DecomposingNormalizerBorrowed!();
        CanonicalCompositions!();
    };
}

macro_rules! ComposingNormalizerBorrowed {
    () => {
        deps!();
        # [doc = " Borrowed version of a normalizer for performing composing normalization."] # [derive (Debug)] pub struct ComposingNormalizerBorrowed < 'a > { decomposing_normalizer : DecomposingNormalizerBorrowed < 'a > , canonical_compositions : & 'a CanonicalCompositions < 'a > , }
    };
}

ComposingNormalizerBorrowed!()