// Generated macro for Response (struct)
macro_rules! Depcrate_client_blocking_io_http_reqwest_remoteResponse {
() => {
// Module: crate::client::blocking_io::http::reqwest::remote
// Provides: {"Response"}
// Dependencies: {}
# [doc = " A link to a thread who provides data for the contained readers."] # [doc = " The expected order is:"] # [doc = " - write `upload_body`"] # [doc = " - read `headers` to end"] # [doc = " - read `body` to hend"] pub (crate) struct Response { pub headers : pipe :: Reader , pub body : pipe :: Reader , pub upload_body : pipe :: Writer , }
};
}
