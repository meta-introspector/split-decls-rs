// Generated macro for impl_534 (impl)
macro_rules! Depcrate_proto_streams_streamsimpl_534 {
() => {
// Module: crate::proto::streams::streams
// Provides: {"impl_534"}
// Dependencies: {}
impl < B , P > Streams < B , P > where P : Peer , { pub fn as_dyn (& self) -> DynStreams < '_ , B > { let Self { inner , send_buffer , _p , } = self ; DynStreams { inner , send_buffer , peer : P :: r#dyn () , } } # [doc = " This function is safe to call multiple times."] # [doc = ""] # [doc = " A `Result` is returned to avoid panicking if the mutex is poisoned."] pub fn recv_eof (& mut self , clear_pending_accept : bool) -> Result < () , () > { self . as_dyn () . recv_eof (clear_pending_accept) } pub (crate) fn max_send_streams (& self) -> usize { self . inner . lock () . unwrap () . counts . max_send_streams () } pub (crate) fn max_recv_streams (& self) -> usize { self . inner . lock () . unwrap () . counts . max_recv_streams () } # [cfg (feature = "unstable")] pub fn num_active_streams (& self) -> usize { let me = self . inner . lock () . unwrap () ; me . store . num_active_streams () } pub fn has_streams (& self) -> bool { let me = self . inner . lock () . unwrap () ; me . counts . has_streams () } pub fn has_streams_or_other_references (& self) -> bool { let me = self . inner . lock () . unwrap () ; me . counts . has_streams () || me . refs > 1 } # [cfg (feature = "unstable")] pub fn num_wired_streams (& self) -> usize { let me = self . inner . lock () . unwrap () ; me . store . num_wired_streams () } }
};
}
