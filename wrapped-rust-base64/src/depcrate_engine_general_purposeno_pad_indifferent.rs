// Generated macro for NO_PAD_INDIFFERENT (const)
macro_rules! Depcrate_engine_general_purposeNO_PAD_INDIFFERENT {
() => {
// Module: crate::engine::general_purpose
// Provides: {"NO_PAD_INDIFFERENT"}
// Dependencies: {}
# [doc = " Don't add padding when encoding, and allow input with or without padding when decoding."] pub const NO_PAD_INDIFFERENT : GeneralPurposeConfig = GeneralPurposeConfig :: new () . with_encode_padding (false) . with_decode_padding_mode (DecodePaddingMode :: Indifferent) ;
};
}
