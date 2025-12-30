// Generated macro for Error (enum)
macro_rules! Depcrate_client_blocking_io_http_curlError {
() => {
// Module: crate::client::blocking_io::http::curl
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by the 'remote' helper, a purely internal construct to perform http requests."] # [doc = ""] # [doc = " It can be used for downcasting errors, which are boxed to hide the actual implementation."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Curl (# [from] curl :: Error) , # [error (transparent)] Redirect (# [from] http :: redirect :: Error) , # [error ("Could not finish reading all data to post to the remote")] ReadPostBody (# [from] std :: io :: Error) , # [error (transparent)] Authenticate (# [from] gix_credentials :: protocol :: Error) , }
};
}
