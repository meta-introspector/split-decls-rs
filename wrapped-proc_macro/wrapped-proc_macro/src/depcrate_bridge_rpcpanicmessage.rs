// Generated macro for PanicMessage (enum)
macro_rules! Depcrate_bridge_rpcPanicMessage {
() => {
// Module: crate::bridge::rpc
// Provides: {"PanicMessage"}
// Dependencies: {}
# [doc = " Simplified version of panic payloads, ignoring"] # [doc = " types other than `&'static str` and `String`."] pub enum PanicMessage { StaticStr (& 'static str) , String (String) , Unknown , }
};
}
