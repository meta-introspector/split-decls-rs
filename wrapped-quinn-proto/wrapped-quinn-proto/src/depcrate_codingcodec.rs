// Generated macro for Codec (trait)
macro_rules! Depcrate_codingCodec {
() => {
// Module: crate::coding
// Provides: {"Codec"}
// Dependencies: {}
# [doc = " Infallible encoding and decoding of QUIC primitives"] pub trait Codec : Sized { # [doc = " Decode a `Self` from the provided buffer, if the buffer is large enough"] fn decode < B : Buf > (buf : & mut B) -> Result < Self > ; # [doc = " Append the encoding of `self` to the provided buffer"] fn encode < B : BufMut > (& self , buf : & mut B) ; }
};
}
