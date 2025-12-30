// Generated macro for impl_124 (impl)
macro_rules! Depcrate_client_blocking_io_http_reqwest_remoteimpl_124 {
() => {
// Module: crate::client::blocking_io::http::reqwest::remote
// Provides: {"impl_124"}
// Dependencies: {}
impl crate :: IsSpuriousError for Error { fn is_spurious (& self) -> bool { match self { Error :: Reqwest (err) => { err . is_timeout () || err . is_connect () || err . status () . is_some_and (| status | status . is_server_error ()) } _ => false , } } }
};
}
