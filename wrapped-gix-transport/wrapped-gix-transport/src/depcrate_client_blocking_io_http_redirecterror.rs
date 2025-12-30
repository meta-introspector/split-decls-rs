// Generated macro for Error (struct)
macro_rules! Depcrate_client_blocking_io_http_redirectError {
() => {
// Module: crate::client::blocking_io::http::redirect
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error provided when redirection went beyond what we deem acceptable."] # [derive (Debug , thiserror :: Error)] # [error ("Redirect url {redirect_url:?} could not be reconciled with original url {expected_url} as they don't share the same suffix")] pub struct Error { redirect_url : String , expected_url : String , }
};
}
