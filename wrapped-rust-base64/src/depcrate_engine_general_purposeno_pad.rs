// Generated macro for NO_PAD (const)
macro_rules! Depcrate_engine_general_purposeNO_PAD {
() => {
// Module: crate::engine::general_purpose
// Provides: {"NO_PAD"}
// Dependencies: {}
# [doc = " Don't add padding when encoding, and require that there is no padding when decoding."] pub const NO_PAD : GeneralPurposeConfig = GeneralPurposeConfig :: new () . with_encode_padding (false) . with_decode_padding_mode (DecodePaddingMode :: RequireNone) ;
};
}
