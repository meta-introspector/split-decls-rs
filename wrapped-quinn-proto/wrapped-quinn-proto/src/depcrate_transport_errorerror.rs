// Generated macro for Error (struct)
macro_rules! Depcrate_transport_errorError {
() => {
// Module: crate::transport_error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Transport-level errors occur when a peer violates the protocol specification"] # [derive (Debug , Clone , Eq , PartialEq)] pub struct Error { # [doc = " Type of error"] pub code : Code , # [doc = " Frame type that triggered the error"] pub frame : Option < frame :: FrameType > , # [doc = " Human-readable explanation of the reason"] pub reason : String , }
};
}
