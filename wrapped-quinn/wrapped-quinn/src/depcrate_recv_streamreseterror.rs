// Generated macro for ResetError (enum)
macro_rules! Depcrate_recv_streamResetError {
() => {
// Module: crate::recv_stream
// Provides: {"ResetError"}
// Dependencies: {}
# [doc = " Errors that arise while waiting for a stream to be reset"] # [derive (Debug , Error , Clone , PartialEq , Eq)] pub enum ResetError { # [doc = " The connection was lost"] # [error ("connection lost")] ConnectionLost (# [from] ConnectionError) , # [doc = " This was a 0-RTT stream and the server rejected it"] # [doc = ""] # [doc = " Can only occur on clients for 0-RTT streams, which can be opened using"] # [doc = " [`Connecting::into_0rtt()`]."] # [doc = ""] # [doc = " [`Connecting::into_0rtt()`]: crate::Connecting::into_0rtt()"] # [error ("0-RTT rejected")] ZeroRttRejected , }
};
}
