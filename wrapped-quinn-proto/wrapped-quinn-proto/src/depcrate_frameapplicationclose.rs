// Generated macro for ApplicationClose (struct)
macro_rules! Depcrate_frameApplicationClose {
() => {
// Module: crate::frame
// Provides: {"ApplicationClose"}
// Dependencies: {}
# [doc = " Reason given by an application for closing the connection"] # [derive (Debug , Clone , PartialEq , Eq)] pub struct ApplicationClose { # [doc = " Application-specific reason code"] pub error_code : VarInt , # [doc = " Human-readable reason for the close"] pub reason : Bytes , }
};
}
