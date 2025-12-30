// Generated macro for impl_172 (impl)
macro_rules! Depcrate_client_blocking_io_bufread_extimpl_172 {
() => {
// Module: crate::client::blocking_io::bufread_ext
// Provides: {"impl_172"}
// Dependencies: {}
impl < 'a , T : ExtendedBufRead < 'a > + ? Sized + 'a > ExtendedBufRead < 'a > for Box < T > { fn set_progress_handler (& mut self , handle_progress : Option < HandleProgress < 'a > >) { self . deref_mut () . set_progress_handler (handle_progress) ; } fn peek_data_line (& mut self) -> Option < io :: Result < Result < & [u8] , Error > > > { self . deref_mut () . peek_data_line () } fn reset (& mut self , version : Protocol) { self . deref_mut () . reset (version) ; } fn stopped_at (& self) -> Option < MessageKind > { self . deref () . stopped_at () } }
};
}
