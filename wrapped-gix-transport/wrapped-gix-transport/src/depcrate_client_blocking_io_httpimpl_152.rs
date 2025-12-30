// Generated macro for impl_152 (impl)
macro_rules! Depcrate_client_blocking_io_httpimpl_152 {
() => {
// Module: crate::client::blocking_io::http
// Provides: {"impl_152"}
// Dependencies: {}
impl < H : Http , B : Unpin > HeadersThenBody < H , B > { fn handle_headers (& mut self) -> std :: io :: Result < () > { if let Some (headers) = self . headers . take () { < Transport < H > > :: check_content_type (self . service , "result" , headers) . map_err (std :: io :: Error :: other) ? ; } Ok (()) } }
};
}
