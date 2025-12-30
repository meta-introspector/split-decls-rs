// Generated macro for ResetStream (struct)
macro_rules! Depcrate_frameResetStream {
() => {
// Module: crate::frame
// Provides: {"ResetStream"}
// Dependencies: {}
# [doc = " A `RESET_STREAM` frame."] # [doc = ""] # [doc = " See [RFC 9000](https://datatracker.ietf.org/doc/html/rfc9000#name-reset_stream-frames) for"] # [doc = " more."] # [derive (Debug , Clone , Eq , PartialEq , Serialize)] pub struct ResetStream { # [doc = " The stream ID over which the RESET_STREAM frame was sent."] pub stream_id : u64 , # [doc = " The error code sent from the peer."] pub error_code : u64 , }
};
}
