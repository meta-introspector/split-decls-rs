// Generated macro for impl_260 (impl)
macro_rules! Depcrate_stream_decoderimpl_260 {
() => {
// Module: crate::stream::decoder
// Provides: {"impl_260"}
// Dependencies: {}
impl < S , P > Decoder < S , P , Bufferless > where P : Default , S : Default , { # [doc = " Constructs a new `Decoder` without an internal buffer. Requires the read instance to be"] # [doc = " wrapped with combine's [`BufReader`] instance to"] # [doc = ""] # [doc = " [`BufReader`]: super::buf_reader::BufReader"] pub fn new_bufferless () -> Self { Decoder :: default () } }
};
}
