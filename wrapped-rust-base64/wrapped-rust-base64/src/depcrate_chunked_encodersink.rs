// Generated macro for Sink (trait)
macro_rules! Depcrate_chunked_encoderSink {
() => {
// Module: crate::chunked_encoder
// Provides: {"Sink"}
// Dependencies: {}
# [doc = " The output mechanism for `ChunkedEncoder`'s encoded bytes."] pub trait Sink { type Error ; # [doc = " Handle a chunk of encoded base64 data (as UTF-8 bytes)"] fn write_encoded_bytes (& mut self , encoded : & [u8]) -> Result < () , Self :: Error > ; }
};
}
