// Generated macro for impl_123 (impl)
macro_rules! Depcrate_driver_process_serverimpl_123 {
() => {
// Module: crate::driver::process::server
// Provides: {"impl_123"}
// Dependencies: {}
# [doc = " Access"] impl Server { # [doc = " Return the list of capabilities we are allowed to use, as negotiated with the client."] pub fn capabilities (& self) -> & HashSet < String > { & self . capabilities } # [doc = " Return the negotiated version of the protocol."] pub fn version (& self) -> usize { self . version } }
};
}
