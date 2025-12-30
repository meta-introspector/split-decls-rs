// Generated macro for ReadableError (enum)
macro_rules! Depcrate_connection_streams_recvReadableError {
() => {
// Module: crate::connection::streams::recv
// Provides: {"ReadableError"}
// Dependencies: {}
# [doc = " Errors triggered when opening a recv stream for reading"] # [derive (Debug , Error , Clone , Eq , PartialEq , Ord , PartialOrd , Hash)] pub enum ReadableError { # [doc = " The stream has not been opened or was already stopped, finished, or reset"] # [error ("closed stream")] ClosedStream , # [doc = " Attempted an ordered read following an unordered read"] # [doc = ""] # [doc = " Performing an unordered read allows discontinuities to arise in the receive buffer of a"] # [doc = " stream which cannot be recovered, making further ordered reads impossible."] # [error ("ordered read after unordered read")] IllegalOrderedRead , }
};
}
