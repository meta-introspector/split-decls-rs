macro_rules! deps {
    () => {
        ComposingNormalizer!();
        CanonicalCompositions!();
    };
}

macro_rules! CanonicalCompositionBorrowed {
    () => {
        deps!();
        # [doc = " Borrowed version of the raw canonical composition operation."] # [doc = ""] # [doc = " Callers should generally use `ComposingNormalizer` instead of this API."] # [doc = " However, this API is provided for callers such as HarfBuzz that specifically"] # [doc = " want access to the raw canonical composition operation e.g. for use in a"] # [doc = " glyph-availability-guided custom normalizer."] # [derive (Debug , Copy , Clone)] pub struct CanonicalCompositionBorrowed < 'a > { canonical_compositions : & 'a CanonicalCompositions < 'a > , }
    };
}

CanonicalCompositionBorrowed!();