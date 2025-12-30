// Generated macro for random_config (function)
macro_rules! Depcrate_testsrandom_config {
() => {
// Module: crate::tests
// Provides: {"random_config"}
// Dependencies: {}
pub fn random_config < R : Rng > (rng : & mut R) -> GeneralPurposeConfig { let mode = rng . gen () ; GeneralPurposeConfig :: new () . with_encode_padding (match mode { DecodePaddingMode :: Indifferent => rng . gen () , DecodePaddingMode :: RequireCanonical => true , DecodePaddingMode :: RequireNone => false , }) . with_decode_padding_mode (mode) . with_decode_allow_trailing_bits (rng . gen ()) }
};
}
