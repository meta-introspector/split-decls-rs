// Generated macro for Error (enum)
macro_rules! Depcrate_fetch_responseError {
() => {
// Module: crate::fetch::response
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned in the [response module][crate::fetch::response]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Failed to read from line reader")] Io (# [source] std :: io :: Error) , # [error (transparent)] UploadPack (# [from] gix_transport :: packetline :: read :: Error) , # [error (transparent)] Transport (# [from] client :: Error) , # [error ("Currently we require feature {feature:?}, which is not supported by the server")] MissingServerCapability { feature : & 'static str } , # [error ("Encountered an unknown line prefix in {line:?}")] UnknownLineType { line : String } , # [error ("Unknown or unsupported header: {header:?}")] UnknownSectionHeader { header : String } , }
};
}
