// Generated macro for GeneralPurposeConfig (struct)
macro_rules! Depcrate_engine_general_purposeGeneralPurposeConfig {
() => {
// Module: crate::engine::general_purpose
// Provides: {"GeneralPurposeConfig"}
// Dependencies: {}
# [doc = " Contains configuration parameters for base64 encoding and decoding."] # [doc = ""] # [doc = " ```"] # [doc = " # use base64::engine::GeneralPurposeConfig;"] # [doc = " let config = GeneralPurposeConfig::new()"] # [doc = "     .with_encode_padding(false);"] # [doc = "     // further customize using `.with_*` methods as needed"] # [doc = " ```"] # [doc = ""] # [doc = " The constants [PAD] and [`NO_PAD`] cover most use cases."] # [doc = ""] # [doc = " To specify the characters used, see [Alphabet]."] # [derive (Clone , Copy , Debug)] pub struct GeneralPurposeConfig { encode_padding : bool , decode_allow_trailing_bits : bool , decode_padding_mode : DecodePaddingMode , }
};
}
