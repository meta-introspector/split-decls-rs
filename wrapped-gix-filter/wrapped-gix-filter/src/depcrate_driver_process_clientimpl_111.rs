// Generated macro for impl_111 (impl)
macro_rules! Depcrate_driver_process_clientimpl_111 {
() => {
// Module: crate::driver::process::client
// Provides: {"impl_111"}
// Dependencies: {}
# [doc = " Access"] impl Client { # [doc = " Return the list of capabilities reported by the serving process."] pub fn capabilities (& self) -> & Capabilities { & self . capabilities } # [doc = " Return the mutable list of capabilities reported by the serving process."] pub fn capabilities_mut (& mut self) -> & mut Capabilities { & mut self . capabilities } # [doc = " Return the negotiated version of the protocol."] # [doc = ""] # [doc = " Note that it is the highest one that both the client and the server support."] pub fn version (& self) -> usize { self . version } }
};
}
