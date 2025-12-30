// Generated macro for impl_808 (impl)
macro_rules! Depcrate_clientimpl_808 {
() => {
// Module: crate::client
// Provides: {"impl_808"}
// Dependencies: {}
impl < T , B > Future for Connection < T , B > where T : AsyncRead + AsyncWrite + Unpin , B : Buf , { type Output = Result < () , crate :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . inner . maybe_close_connection_if_no_streams () ; let had_streams_or_refs = self . inner . has_streams_or_other_references () ; let result = self . inner . poll (cx) . map_err (Into :: into) ; if result . is_pending () && had_streams_or_refs && ! self . inner . has_streams_or_other_references () { tracing :: trace ! ("last stream closed during poll, wake again") ; cx . waker () . wake_by_ref () ; } result } }
};
}
