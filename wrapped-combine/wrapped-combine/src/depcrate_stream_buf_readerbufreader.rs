// Generated macro for BufReader (struct)
macro_rules! Depcrate_stream_buf_readerBufReader {
() => {
// Module: crate::stream::buf_reader
// Provides: {"BufReader"}
// Dependencies: {}
# [cfg (not (feature = "pin-project-lite"))] # [doc = " `BufReader` used by `Decoder` when it is constructed with [`Decoder::new_bufferless`][]"] # [doc = ""] # [doc = " [`Decoder::new_bufferless`]: ../decoder/struct.Decoder.html#method.new_bufferless"] # [derive (Debug)] pub struct BufReader < R > { inner : R , buf : BytesMut , }
};
}
