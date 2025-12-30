// Generated macro for Error (enum)
macro_rules! Depcrate_client_blocking_io_http_reqwest_remoteError {
() => {
// Module: crate::client::blocking_io::http::reqwest::remote
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error returned by the 'remote' helper, a purely internal construct to perform http requests."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error (transparent)] Reqwest (# [from] reqwest :: Error) , # [error ("Could not finish reading all data to post to the remote")] ReadPostBody (# [from] std :: io :: Error) , # [error ("Request configuration failed")] ConfigureRequest (# [from] Box < dyn std :: error :: Error + Send + Sync + 'static >) , # [error (transparent)] Redirect (# [from] redirect :: Error) , }
};
}
