// Generated macro for ByteSlice (struct)
macro_rules! Depcrate_connection_streams_sendByteSlice {
() => {
// Module: crate::connection::streams::send
// Provides: {"ByteSlice"}
// Dependencies: {}
# [doc = " A [`BytesSource`] implementation for `&[u8]`"] # [doc = ""] # [doc = " The type allows to dequeue a single [`Bytes`] chunk, which will be lazily"] # [doc = " created from a reference. This allows to defer the allocation until it is"] # [doc = " known how much data needs to be copied."] pub (crate) struct ByteSlice < 'a > { # [doc = " The wrapped byte slice"] data : & 'a [u8] , }
};
}
