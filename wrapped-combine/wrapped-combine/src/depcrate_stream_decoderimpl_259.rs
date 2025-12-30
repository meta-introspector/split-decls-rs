// Generated macro for impl_259 (impl)
macro_rules! Depcrate_stream_decoderimpl_259 {
() => {
// Module: crate::stream::decoder
// Provides: {"impl_259"}
// Dependencies: {}
impl < S , P > Decoder < S , P , Buffer > where P : Default , S : Default , { # [doc = " Constructs a new [`Decoder`] with an internal buffer. Allows any `AsyncRead/Read` instance to"] # [doc = " be used when decoding but there may be data left in the internal buffer after decoding"] # [doc = " (accessible with [`Decoder::buffer`])"] pub fn new () -> Self { Decoder :: default () } # [doc = " Constructs a new [`Decoder`] with an internal buffer. Allows any `AsyncRead/Read` instance to"] # [doc = " be used when decoding but there may be data left in the internal buffer after decoding"] # [doc = " (accessible with [`Decoder::buffer`])"] pub fn new_buffer () -> Self { Decoder :: new () } }
};
}
