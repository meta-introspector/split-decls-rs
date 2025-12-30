// Generated macro for impl_145 (impl)
macro_rules! Depcrate_client_blocking_io_httpimpl_145 {
() => {
// Module: crate::client::blocking_io::http
// Provides: {"impl_145"}
// Dependencies: {}
impl < H : Http > Transport < H > { # [doc = " Returns the identity that the transport uses when connecting to the remote."] pub fn identity (& self) -> Option < & gix_sec :: identity :: Account > { self . identity . as_ref () } }
};
}
