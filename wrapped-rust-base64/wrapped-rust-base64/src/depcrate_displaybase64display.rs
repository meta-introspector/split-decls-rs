// Generated macro for Base64Display (struct)
macro_rules! Depcrate_displayBase64Display {
() => {
// Module: crate::display
// Provides: {"Base64Display"}
// Dependencies: {}
# [doc = " A convenience wrapper for base64'ing bytes into a format string without heap allocation."] pub struct Base64Display < 'a , 'e , E : Engine > { bytes : & 'a [u8] , chunked_encoder : ChunkedEncoder < 'e , E > , }
};
}
