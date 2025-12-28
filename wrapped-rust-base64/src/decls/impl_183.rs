macro_rules! deps {
    () => {
        Engine!();
        GeneralPurposeConfig!();
        GeneralPurpose!();
        Alphabet!();
        EngineWrapper!();
        DecodePaddingMode!();
        GeneralPurposeWrapper!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        impl EngineWrapper for GeneralPurposeWrapper { type Engine = general_purpose :: GeneralPurpose ; fn standard () -> Self :: Engine { general_purpose :: GeneralPurpose :: new (& STANDARD , general_purpose :: PAD) } fn standard_unpadded () -> Self :: Engine { general_purpose :: GeneralPurpose :: new (& STANDARD , general_purpose :: NO_PAD) } fn standard_with_pad_mode (encode_pad : bool , decode_pad_mode : DecodePaddingMode ,) -> Self :: Engine { general_purpose :: GeneralPurpose :: new (& STANDARD , general_purpose :: GeneralPurposeConfig :: new () . with_encode_padding (encode_pad) . with_decode_padding_mode (decode_pad_mode) ,) } fn standard_allow_trailing_bits () -> Self :: Engine { general_purpose :: GeneralPurpose :: new (& STANDARD , general_purpose :: GeneralPurposeConfig :: new () . with_decode_allow_trailing_bits (true) ,) } fn random < R : rand :: Rng > (rng : & mut R) -> Self :: Engine { let alphabet = random_alphabet (rng) ; Self :: random_alphabet (rng , alphabet) } fn random_alphabet < R : rand :: Rng > (rng : & mut R , alphabet : & Alphabet) -> Self :: Engine { general_purpose :: GeneralPurpose :: new (alphabet , random_config (rng)) } }
    };
}

impl_183!()