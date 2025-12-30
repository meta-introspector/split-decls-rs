// Generated macro for Error (enum)
macro_rules! Depcrate_client_blocking_io_http_traitsError {
() => {
// Module: crate::client::blocking_io::http::traits
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The error used by the [Http] trait."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Could not initialize the http client")] InitHttpClient { source : Box < dyn std :: error :: Error + Send + Sync + 'static > , } , # [error ("{description}")] Detail { description : String } , # [error ("An IO error occurred while uploading the body of a POST request")] PostBody (# [from] std :: io :: Error) , }
};
}
