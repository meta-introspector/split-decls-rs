// Generated macro for impl_155 (impl)
macro_rules! Depcrate_client_blocking_io_httpimpl_155 {
() => {
// Module: crate::client::blocking_io::http
// Provides: {"impl_155"}
// Dependencies: {}
impl < H : Http , B : ReadlineBufRead + Unpin > ReadlineBufRead for HeadersThenBody < H , B > { fn readline (& mut self) -> Option < std :: io :: Result < Result < PacketLineRef < '_ > , gix_packetline :: decode :: Error > > > { if let Err (err) = self . handle_headers () { return Some (Err (err)) ; } self . body . readline () } fn readline_str (& mut self , line : & mut String) -> std :: io :: Result < usize > { self . handle_headers () ? ; self . body . readline_str (line) } }
};
}
