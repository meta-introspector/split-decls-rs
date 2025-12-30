// Generated macro for impl_113 (impl)
macro_rules! Depcrate_client_blocking_io_http_curlimpl_113 {
() => {
// Module: crate::client::blocking_io::http::curl
// Provides: {"impl_113"}
// Dependencies: {}
impl Default for Curl { fn default () -> Self { let (handle , req , res) = remote :: new () ; Curl { handle : Some (handle) , req , res , config : http :: Options :: default () , } } }
};
}
