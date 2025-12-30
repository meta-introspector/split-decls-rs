// Generated macro for PAD_INDIFFERENT (const)
macro_rules! Depcrate_engine_general_purposePAD_INDIFFERENT {
() => {
// Module: crate::engine::general_purpose
// Provides: {"PAD_INDIFFERENT"}
// Dependencies: {}
# [doc = " Include padding bytes when encoding, but allow input with or without padding when decoding."] pub const PAD_INDIFFERENT : GeneralPurposeConfig = GeneralPurposeConfig :: new () . with_encode_padding (true) . with_decode_padding_mode (DecodePaddingMode :: Indifferent) ;
};
}
