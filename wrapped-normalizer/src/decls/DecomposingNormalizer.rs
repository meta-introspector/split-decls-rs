macro_rules! DecomposingNormalizer {
    () => {
        # [doc = " A normalizer for performing decomposing normalization."] # [derive (Debug)] pub struct DecomposingNormalizer { decompositions : DataPayload < NormalizerNfdDataV1 > , tables : DataPayload < NormalizerNfdTablesV1 > , supplementary_tables : Option < DataPayload < NormalizerNfkdTablesV1 > > , decomposition_passthrough_bound : u8 , composition_passthrough_bound : u16 , }
    };
}

DecomposingNormalizer!();