macro_rules! deps {
    () => {
        GeneralPurposeConfig!();
        DecodePaddingMode!();
    };
}

macro_rules! random_config {
    () => {
        deps!();
        pub fn random_config < R : Rng > (rng : & mut R) -> GeneralPurposeConfig { let mode = rng . gen () ; GeneralPurposeConfig :: new () . with_encode_padding (match mode { DecodePaddingMode :: Indifferent => rng . gen () , DecodePaddingMode :: RequireCanonical => true , DecodePaddingMode :: RequireNone => false , }) . with_decode_padding_mode (mode) . with_decode_allow_trailing_bits (rng . gen ()) }
    };
}

random_config!()