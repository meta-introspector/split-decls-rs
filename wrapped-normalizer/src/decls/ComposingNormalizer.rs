macro_rules! deps {
    () => {
        DecomposingNormalizer!();
    };
}

macro_rules! ComposingNormalizer {
    () => {
        deps!();
        # [doc = " A normalizer for performing composing normalization."] # [derive (Debug)] pub struct ComposingNormalizer { decomposing_normalizer : DecomposingNormalizer , canonical_compositions : DataPayload < NormalizerNfcV1 > , }
    };
}

ComposingNormalizer!();