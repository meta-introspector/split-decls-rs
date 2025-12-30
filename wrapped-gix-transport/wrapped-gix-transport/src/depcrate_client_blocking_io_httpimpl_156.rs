// Generated macro for impl_156 (impl)
macro_rules! Depcrate_client_blocking_io_httpimpl_156 {
() => {
// Module: crate::client::blocking_io::http
// Provides: {"impl_156"}
// Dependencies: {}
impl < 'a , H : Http , B : ExtendedBufRead < 'a > + Unpin > ExtendedBufRead < 'a > for HeadersThenBody < H , B > { fn set_progress_handler (& mut self , handle_progress : Option < HandleProgress < 'a > >) { self . body . set_progress_handler (handle_progress) ; } fn peek_data_line (& mut self) -> Option < std :: io :: Result < Result < & [u8] , client :: Error > > > { if let Err (err) = self . handle_headers () { return Some (Err (err)) ; } self . body . peek_data_line () } fn reset (& mut self , version : Protocol) { self . body . reset (version) ; } fn stopped_at (& self) -> Option < MessageKind > { self . body . stopped_at () } }
};
}
