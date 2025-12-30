// Generated macro for impl_153 (impl)
macro_rules! Depcrate_client_blocking_io_httpimpl_153 {
() => {
// Module: crate::client::blocking_io::http
// Provides: {"impl_153"}
// Dependencies: {}
impl < H : Http , B : Read + Unpin > Read for HeadersThenBody < H , B > { fn read (& mut self , buf : & mut [u8]) -> std :: io :: Result < usize > { self . handle_headers () ? ; self . body . read (buf) } }
};
}
