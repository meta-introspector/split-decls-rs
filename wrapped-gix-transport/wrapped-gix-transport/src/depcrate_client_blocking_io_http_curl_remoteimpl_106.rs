// Generated macro for impl_106 (impl)
macro_rules! Depcrate_client_blocking_io_http_curl_remoteimpl_106 {
() => {
// Module: crate::client::blocking_io::http::curl::remote
// Provides: {"impl_106"}
// Dependencies: {}
impl From < curl :: Error > for http :: Error { fn from (err : curl :: Error) -> Self { http :: Error :: Detail { description : err . to_string () , } } }
};
}
