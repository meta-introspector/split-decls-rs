// Generated macro for impl_154 (impl)
macro_rules! Depcrate_client_blocking_io_httpimpl_154 {
() => {
// Module: crate::client::blocking_io::http
// Provides: {"impl_154"}
// Dependencies: {}
impl < H : Http , B : BufRead + Unpin > BufRead for HeadersThenBody < H , B > { fn fill_buf (& mut self) -> std :: io :: Result < & [u8] > { self . handle_headers () ? ; self . body . fill_buf () } fn consume (& mut self , amt : usize) { self . body . consume (amt) ; } }
};
}
