macro_rules! deps {
    () => {
        DecompositionData!();
        DecompositionTables!();
    };
}

macro_rules! DecomposingNormalizerBorrowed {
    () => {
        deps!();
        # [doc = " Borrowed version of a normalizer for performing decomposing normalization."] # [derive (Debug)] pub struct DecomposingNormalizerBorrowed < 'a > { decompositions : & 'a DecompositionData < 'a > , tables : & 'a DecompositionTables < 'a > , supplementary_tables : Option < & 'a DecompositionTables < 'a > > , decomposition_passthrough_bound : u8 , composition_passthrough_bound : u16 , }
    };
}

DecomposingNormalizerBorrowed!();