// Generated macro for DecomposingNormalizerBorrowed (struct)
macro_rules! DepcrateDecomposingNormalizerBorrowed {
() => {
// Module: crate
// Provides: {"DecomposingNormalizerBorrowed"}
// Dependencies: {}
# [doc = " Borrowed version of a normalizer for performing decomposing normalization."] # [derive (Debug)] pub struct DecomposingNormalizerBorrowed < 'a > { decompositions : & 'a DecompositionData < 'a > , tables : & 'a DecompositionTables < 'a > , supplementary_tables : Option < & 'a DecompositionTables < 'a > > , decomposition_passthrough_bound : u8 , composition_passthrough_bound : u16 , }
};
}
