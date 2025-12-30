// Generated macro for FinishError (enum)
macro_rules! Depcrate_connection_streams_sendFinishError {
() => {
// Module: crate::connection::streams::send
// Provides: {"FinishError"}
// Dependencies: {}
# [doc = " Reasons why attempting to finish a stream might fail"] # [derive (Debug , Error , Clone , PartialEq , Eq)] pub enum FinishError { # [doc = " The peer is no longer accepting data on this stream. No"] # [doc = " [`StreamEvent::Finished`] event will be emitted for this stream."] # [doc = ""] # [doc = " Carries an application-defined error code."] # [doc = ""] # [doc = " [`StreamEvent::Finished`]: crate::StreamEvent::Finished"] # [error ("stopped by peer: code {0}")] Stopped (VarInt) , # [doc = " The stream has not been opened or was already finished or reset"] # [error ("closed stream")] ClosedStream , }
};
}
