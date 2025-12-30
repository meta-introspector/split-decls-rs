// Generated macro for macro_98 (macro)
macro_rules! Depcrate_stream_buf_readermacro_98 {
() => {
// Module: crate::stream::buf_reader
// Provides: {"macro_98"}
// Dependencies: {}
# [cfg (feature = "pin-project-lite")] pin_project ! { # [doc = " `BufReader` used by `Decoder` when it is constructed with [`Decoder::new_bufferless`][]"] # [doc = ""] # [doc = " [`Decoder::new_bufferless`]: ../decoder/struct.Decoder.html#method.new_bufferless"] # [derive (Debug)] pub struct BufReader < R > { # [pin] inner : R , buf : BytesMut } }
};
}
