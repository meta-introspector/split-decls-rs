// Generated macro for WriteError (enum)
macro_rules! Depcrate_connection_streams_sendWriteError {
() => {
// Module: crate::connection::streams::send
// Provides: {"WriteError"}
// Dependencies: {}
# [doc = " Errors triggered while writing to a send stream"] # [derive (Debug , Error , Clone , Eq , PartialEq , Ord , PartialOrd , Hash)] pub enum WriteError { # [doc = " The peer is not able to accept additional data, or the connection is congested."] # [doc = ""] # [doc = " If the peer issues additional flow control credit, a [`StreamEvent::Writable`] event will"] # [doc = " be generated, indicating that retrying the write might succeed."] # [doc = ""] # [doc = " [`StreamEvent::Writable`]: crate::StreamEvent::Writable"] # [error ("unable to accept further writes")] Blocked , # [doc = " The peer is no longer accepting data on this stream, and it has been implicitly reset. The"] # [doc = " stream cannot be finished or further written to."] # [doc = ""] # [doc = " Carries an application-defined error code."] # [doc = ""] # [doc = " [`StreamEvent::Finished`]: crate::StreamEvent::Finished"] # [error ("stopped by peer: code {0}")] Stopped (VarInt) , # [doc = " The stream has not been opened or has already been finished or reset"] # [error ("closed stream")] ClosedStream , }
};
}
