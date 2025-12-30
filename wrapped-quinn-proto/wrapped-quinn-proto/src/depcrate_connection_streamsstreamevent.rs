// Generated macro for StreamEvent (enum)
macro_rules! Depcrate_connection_streamsStreamEvent {
() => {
// Module: crate::connection::streams
// Provides: {"StreamEvent"}
// Dependencies: {}
# [doc = " Application events about streams"] # [derive (Debug , PartialEq , Eq)] pub enum StreamEvent { # [doc = " One or more new streams has been opened and might be readable"] Opened { # [doc = " Directionality for which streams have been opened"] dir : Dir , } , # [doc = " A currently open stream likely has data or errors waiting to be read"] Readable { # [doc = " Which stream is now readable"] id : StreamId , } , # [doc = " A formerly write-blocked stream might be ready for a write or have been stopped"] # [doc = ""] # [doc = " Only generated for streams that are currently open."] Writable { # [doc = " Which stream is now writable"] id : StreamId , } , # [doc = " A finished stream has been fully acknowledged or stopped"] Finished { # [doc = " Which stream has been finished"] id : StreamId , } , # [doc = " The peer asked us to stop sending on an outgoing stream"] Stopped { # [doc = " Which stream has been stopped"] id : StreamId , # [doc = " Error code supplied by the peer"] error_code : VarInt , } , # [doc = " At least one new stream of a certain directionality may be opened"] Available { # [doc = " Directionality for which streams are newly available"] dir : Dir , } , }
};
}
