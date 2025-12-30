// Generated macro for BytesOpts (struct)
macro_rules! Depcrate_driverBytesOpts {
() => {
// Module: crate::driver
// Provides: {"BytesOpts"}
// Dependencies: {}
# [doc = " Options for choosing a character encoding"] # [derive (Clone , Default)] pub struct BytesOpts { # [doc = " The character encoding specified by the transport layer, if any."] # [doc = " In HTTP for example, this is the `charset` parameter of the `Content-Type` response header."] pub transport_layer_encoding : Option < EncodingRef > , }
};
}
