// Generated macro for decode (function)
macro_rules! Depcrate_encodingdecode {
() => {
// Module: crate::encoding
// Provides: {"decode"}
// Dependencies: {}
# [doc = " Decodes the provided bytes using the specified encoding."] # [doc = ""] # [doc = " Returns an error in case of malformed or non-representable sequences in the `bytes`."] # [cfg (feature = "encoding")] pub fn decode < 'b > (bytes : & 'b [u8] , encoding : & 'static Encoding ,) -> Result < Cow < 'b , str > , EncodingError > { encoding . decode_without_bom_handling_and_without_replacement (bytes) . ok_or (EncodingError :: Other (encoding)) }
};
}
