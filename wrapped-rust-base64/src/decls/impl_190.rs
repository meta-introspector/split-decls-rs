macro_rules! deps {
    () => {
        DecoderReaderEngine!();
        GeneralPurposeWrapper!();
        DecoderReaderEngineWrapper!();
        DecodePaddingMode!();
        Alphabet!();
        Engine!();
        EngineWrapper!();
        GeneralPurpose!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl EngineWrapper for DecoderReaderEngineWrapper { type Engine = DecoderReaderEngine < general_purpose :: GeneralPurpose > ; fn standard () -> Self :: Engine { GeneralPurposeWrapper :: standard () . into () } fn standard_unpadded () -> Self :: Engine { GeneralPurposeWrapper :: standard_unpadded () . into () } fn standard_with_pad_mode (encode_pad : bool , decode_pad_mode : DecodePaddingMode ,) -> Self :: Engine { GeneralPurposeWrapper :: standard_with_pad_mode (encode_pad , decode_pad_mode) . into () } fn standard_allow_trailing_bits () -> Self :: Engine { GeneralPurposeWrapper :: standard_allow_trailing_bits () . into () } fn random < R : rand :: Rng > (rng : & mut R) -> Self :: Engine { GeneralPurposeWrapper :: random (rng) . into () } fn random_alphabet < R : rand :: Rng > (rng : & mut R , alphabet : & Alphabet) -> Self :: Engine { GeneralPurposeWrapper :: random_alphabet (rng , alphabet) . into () } }
    };
}

impl_190!();