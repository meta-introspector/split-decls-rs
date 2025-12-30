// Generated macro for SendStream (struct)
macro_rules! Depcrate_send_streamSendStream {
() => {
// Module: crate::send_stream
// Provides: {"SendStream"}
// Dependencies: {}
# [doc = " A stream that can only be used to send data"] # [doc = ""] # [doc = " If dropped, streams that haven't been explicitly [`reset()`] will be implicitly [`finish()`]ed,"] # [doc = " continuing to (re)transmit previously written data until it has been fully acknowledged or the"] # [doc = " connection is closed."] # [doc = ""] # [doc = " # Cancellation"] # [doc = ""] # [doc = " A `write` method is said to be *cancel-safe* when dropping its future before the future becomes"] # [doc = " ready will always result in no data being written to the stream. This is true of methods which"] # [doc = " succeed immediately when any progress is made, and is not true of methods which might need to"] # [doc = " perform multiple writes internally before succeeding. Each `write` method documents whether it is"] # [doc = " cancel-safe."] # [doc = ""] # [doc = " [`reset()`]: SendStream::reset"] # [doc = " [`finish()`]: SendStream::finish"] # [derive (Debug)] pub struct SendStream { conn : ConnectionRef , stream : StreamId , is_0rtt : bool , }
};
}
