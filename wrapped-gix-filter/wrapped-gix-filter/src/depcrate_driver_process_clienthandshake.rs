// Generated macro for handshake (module)
macro_rules! Depcrate_driver_process_clienthandshake {
() => {
// Module: crate::driver::process::client
// Provides: {"handshake"}
// Dependencies: {}
# [doc = ""] pub mod handshake { # [doc = " The error returned by [Client::handshake()][super::Client::handshake()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Failed to read or write to the process")] Io (# [from] std :: io :: Error) , # [error ("{msg} '{actual}'")] Protocol { msg : String , actual : String } , # [error ("The server sent the '{name}' capability which isn't among the ones we desire can support")] UnsupportedCapability { name : String } , } }
};
}
