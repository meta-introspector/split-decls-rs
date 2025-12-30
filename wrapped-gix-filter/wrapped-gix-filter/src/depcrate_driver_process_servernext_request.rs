// Generated macro for next_request (module)
macro_rules! Depcrate_driver_process_servernext_request {
() => {
// Module: crate::driver::process::server
// Provides: {"next_request"}
// Dependencies: {}
# [doc = ""] pub mod next_request { use bstr :: BString ; # [doc = " The error returned by [Server::next_request()][super::Server::next_request()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Failed to read from the client")] Io (# [from] std :: io :: Error) , # [error ("{msg} '{actual}'")] Protocol { msg : String , actual : BString } , # [error (transparent)] PacketlineDecode (# [from] gix_packetline :: decode :: Error) , } }
};
}
