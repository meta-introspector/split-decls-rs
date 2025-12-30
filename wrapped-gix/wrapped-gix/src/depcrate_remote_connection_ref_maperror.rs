// Generated macro for Error (enum)
macro_rules! Depcrate_remote_connection_ref_mapError {
() => {
// Module: crate::remote::connection::ref_map
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by [`Connection::ref_map()`]."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] InitRefMap (# [from] gix_protocol :: fetch :: refmap :: init :: Error) , # [error ("Failed to configure the transport before connecting to {url:?}")] GatherTransportConfig { url : BString , source : crate :: config :: transport :: Error , } , # [error ("Failed to configure the transport layer")] ConfigureTransport (# [from] Box < dyn std :: error :: Error + Send + Sync + 'static >) , # [error (transparent)] Handshake (# [from] gix_protocol :: handshake :: Error) , # [error (transparent)] Transport (# [from] gix_protocol :: transport :: client :: Error) , # [error (transparent)] ConfigureCredentials (# [from] crate :: config :: credential_helpers :: Error) , }
};
}
