// Generated macro for Options (struct)
macro_rules! Depcrate_client_blocking_io_http_reqwestOptions {
() => {
// Module: crate::client::blocking_io::http::reqwest
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Options to configure the reqwest HTTP handler."] # [derive (Default)] pub struct Options { # [doc = " A function to configure the request that is about to be made."] pub configure_request : Option < Box < ConfigureRequestFn > > , }
};
}
