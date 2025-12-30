// Generated macro for ReadError (enum)
macro_rules! Depcrate_recv_streamReadError {
() => {
// Module: crate::recv_stream
// Provides: {"ReadError"}
// Dependencies: {}
# [doc = " Errors that arise from reading from a stream."] # [derive (Debug , Error , Clone , PartialEq , Eq)] pub enum ReadError { # [doc = " The peer abandoned transmitting data on this stream"] # [doc = ""] # [doc = " Carries an application-defined error code."] # [error ("stream reset by peer: error {0}")] Reset (VarInt) , # [doc = " The connection was lost"] # [error ("connection lost")] ConnectionLost (# [from] ConnectionError) , # [doc = " The stream has already been stopped, finished, or reset"] # [error ("closed stream")] ClosedStream , # [doc = " Attempted an ordered read following an unordered read"] # [doc = ""] # [doc = " Performing an unordered read allows discontinuities to arise in the receive buffer of a"] # [doc = " stream which cannot be recovered, making further ordered reads impossible."] # [error ("ordered read after unordered read")] IllegalOrderedRead , # [doc = " This was a 0-RTT stream and the server rejected it"] # [doc = ""] # [doc = " Can only occur on clients for 0-RTT streams, which can be opened using"] # [doc = " [`Connecting::into_0rtt()`]."] # [doc = ""] # [doc = " [`Connecting::into_0rtt()`]: crate::Connecting::into_0rtt()"] # [error ("0-RTT rejected")] ZeroRttRejected , }
};
}
