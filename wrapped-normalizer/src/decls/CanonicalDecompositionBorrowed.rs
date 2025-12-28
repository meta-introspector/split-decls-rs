macro_rules! deps {
    () => {
        DecompositionTables!();
        NonRecursiveDecompositionSupplement!();
        DecompositionData!();
        DecomposingNormalizer!();
    };
}

macro_rules! CanonicalDecompositionBorrowed {
    () => {
        deps!();
        # [doc = " Borrowed version of the raw (non-recursive) canonical decomposition operation."] # [doc = ""] # [doc = " Callers should generally use `DecomposingNormalizer` instead of this API."] # [doc = " However, this API is provided for callers such as HarfBuzz that specifically"] # [doc = " want access to non-recursive canonical decomposition e.g. for use in a"] # [doc = " glyph-availability-guided custom normalizer."] # [derive (Debug)] pub struct CanonicalDecompositionBorrowed < 'a > { decompositions : & 'a DecompositionData < 'a > , tables : & 'a DecompositionTables < 'a > , non_recursive : & 'a NonRecursiveDecompositionSupplement < 'a > , }
    };
}

CanonicalDecompositionBorrowed!()