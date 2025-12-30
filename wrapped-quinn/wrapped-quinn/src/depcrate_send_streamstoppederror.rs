// Generated macro for StoppedError (enum)
macro_rules! Depcrate_send_streamStoppedError {
() => {
// Module: crate::send_stream
// Provides: {"StoppedError"}
// Dependencies: {}
# [doc = " Errors that arise while monitoring for a send stream stop from the peer"] # [derive (Debug , Error , Clone , PartialEq , Eq)] pub enum StoppedError { # [doc = " The connection was lost"] # [error ("connection lost")] ConnectionLost (# [from] ConnectionError) , # [doc = " This was a 0-RTT stream and the server rejected it"] # [doc = ""] # [doc = " Can only occur on clients for 0-RTT streams, which can be opened using"] # [doc = " [`Connecting::into_0rtt()`]."] # [doc = ""] # [doc = " [`Connecting::into_0rtt()`]: crate::Connecting::into_0rtt()"] # [error ("0-RTT rejected")] ZeroRttRejected , }
};
}
