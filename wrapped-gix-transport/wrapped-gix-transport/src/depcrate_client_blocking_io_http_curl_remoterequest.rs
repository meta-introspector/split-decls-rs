// Generated macro for Request (struct)
macro_rules! Depcrate_client_blocking_io_http_curl_remoteRequest {
() => {
// Module: crate::client::blocking_io::http::curl::remote
// Provides: {"Request"}
// Dependencies: {}
pub struct Request { pub url : String , pub base_url : String , pub headers : curl :: easy :: List , pub upload_body_kind : Option < PostBodyDataKind > , pub config : http :: Options , }
};
}
