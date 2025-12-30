// Generated macro for impl_109 (impl)
macro_rules! Depcrate_client_blocking_io_http_curlimpl_109 {
() => {
// Module: crate::client::blocking_io::http::curl
// Provides: {"impl_109"}
// Dependencies: {}
impl crate :: IsSpuriousError for Error { fn is_spurious (& self) -> bool { match self { Error :: Curl (err) => curl_is_spurious (err) , _ => false , } } }
};
}
