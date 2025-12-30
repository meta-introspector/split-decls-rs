// Generated macro for invoke (module)
macro_rules! Depcrate_driver_process_clientinvoke {
() => {
// Module: crate::driver::process::client
// Provides: {"invoke"}
// Dependencies: {}
# [doc = ""] pub mod invoke { # [doc = " The error returned by [Client::invoke()][super::Client::invoke()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Failed to read or write to the process")] Io (# [from] std :: io :: Error) , } # [doc = ""] pub mod without_content { # [doc = " The error returned by [Client::invoke_without_content()][super::super::Client::invoke_without_content()]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Failed to read or write to the process")] Io (# [from] std :: io :: Error) , # [error (transparent)] PacketlineDecode (# [from] gix_packetline :: decode :: Error) , } impl From < super :: Error > for Error { fn from (value : super :: Error) -> Self { match value { super :: Error :: Io (err) => Error :: Io (err) , } } } } }
};
}
