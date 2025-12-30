// Generated macro for WriteError (enum)
macro_rules! Depcrate_send_streamWriteError {
() => {
// Module: crate::send_stream
// Provides: {"WriteError"}
// Dependencies: {}
# [doc = " Errors that arise from writing to a stream"] # [derive (Debug , Error , Clone , PartialEq , Eq)] pub enum WriteError { # [doc = " The peer is no longer accepting data on this stream"] # [doc = ""] # [doc = " Carries an application-defined error code."] # [error ("sending stopped by peer: error {0}")] Stopped (VarInt) , # [doc = " The connection was lost"] # [error ("connection lost")] ConnectionLost (# [from] ConnectionError) , # [doc = " The stream has already been finished or reset"] # [error ("closed stream")] ClosedStream , # [doc = " This was a 0-RTT stream and the server rejected it"] # [doc = ""] # [doc = " Can only occur on clients for 0-RTT streams, which can be opened using"] # [doc = " [`Connecting::into_0rtt()`]."] # [doc = ""] # [doc = " [`Connecting::into_0rtt()`]: crate::Connecting::into_0rtt()"] # [error ("0-RTT rejected")] ZeroRttRejected , }
};
}
