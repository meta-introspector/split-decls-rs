// Generated macro for BytesArray (struct)
macro_rules! Depcrate_connection_streams_sendBytesArray {
() => {
// Module: crate::connection::streams::send
// Provides: {"BytesArray"}
// Dependencies: {}
# [doc = " A [`BytesSource`] implementation for `&'a mut [Bytes]`"] # [doc = ""] # [doc = " The type allows to dequeue [`Bytes`] chunks from an array of chunks, up to"] # [doc = " a configured limit."] pub (crate) struct BytesArray < 'a > { # [doc = " The wrapped slice of `Bytes`"] chunks : & 'a mut [Bytes] , # [doc = " The amount of chunks consumed from this source"] consumed : usize , }
};
}
