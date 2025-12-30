// Generated macro for impl_21 (impl)
macro_rules! Depcrate_client_async_io_bufread_extimpl_21 {
() => {
// Module: crate::client::async_io::bufread_ext
// Provides: {"impl_21"}
// Dependencies: {}
# [async_trait (? Send)] impl < 'a , T : ExtendedBufRead < 'a > + ? Sized + 'a + Unpin > ExtendedBufRead < 'a > for Box < T > { fn set_progress_handler (& mut self , handle_progress : Option < HandleProgress < 'a > >) { self . deref_mut () . set_progress_handler (handle_progress) ; } async fn peek_data_line (& mut self) -> Option < io :: Result < Result < & [u8] , Error > > > { self . deref_mut () . peek_data_line () . await } fn reset (& mut self , version : Protocol) { self . deref_mut () . reset (version) ; } fn stopped_at (& self) -> Option < MessageKind > { self . deref () . stopped_at () } }
};
}
