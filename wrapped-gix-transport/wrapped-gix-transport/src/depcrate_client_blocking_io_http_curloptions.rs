// Generated macro for Options (struct)
macro_rules! Depcrate_client_blocking_io_http_curlOptions {
() => {
// Module: crate::client::blocking_io::http::curl
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Options to configure the `curl` HTTP handler."] # [derive (Default)] pub struct Options { # [doc = " If `true` and runtime configuration is possible for `curl` backends, certificates revocation will be checked."] # [doc = ""] # [doc = " This only works on windows apparently. Ignored if `None`."] pub schannel_check_revoke : Option < bool > , }
};
}
