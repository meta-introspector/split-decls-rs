// Generated macro for ReadError (enum)
macro_rules! Depcrate_connection_streams_recvReadError {
() => {
// Module: crate::connection::streams::recv
// Provides: {"ReadError"}
// Dependencies: {}
# [doc = " Errors triggered when reading from a recv stream"] # [derive (Debug , Error , Clone , Eq , PartialEq , Ord , PartialOrd , Hash)] pub enum ReadError { # [doc = " No more data is currently available on this stream."] # [doc = ""] # [doc = " If more data on this stream is received from the peer, an `Event::StreamReadable` will be"] # [doc = " generated for this stream, indicating that retrying the read might succeed."] # [error ("blocked")] Blocked , # [doc = " The peer abandoned transmitting data on this stream."] # [doc = ""] # [doc = " Carries an application-defined error code."] # [error ("reset by peer: code {0}")] Reset (VarInt) , }
};
}
