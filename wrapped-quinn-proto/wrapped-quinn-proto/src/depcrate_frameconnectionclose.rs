// Generated macro for ConnectionClose (struct)
macro_rules! Depcrate_frameConnectionClose {
() => {
// Module: crate::frame
// Provides: {"ConnectionClose"}
// Dependencies: {}
# [doc = " Reason given by the transport for closing the connection"] # [derive (Debug , Clone , PartialEq , Eq)] pub struct ConnectionClose { # [doc = " Class of error as encoded in the specification"] pub error_code : TransportErrorCode , # [doc = " Type of frame that caused the close"] pub frame_type : Option < FrameType > , # [doc = " Human-readable reason for the close"] pub reason : Bytes , }
};
}
