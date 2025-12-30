// Generated macro for impl_1846 (impl)
macro_rules! Depcrate_sink_closeimpl_1846 {
() => {
// Module: crate::sink::close
// Provides: {"impl_1846"}
// Dependencies: {}
impl < Si : Sink < Item > + Unpin + ? Sized , Item > Future for Close < '_ , Si , Item > { type Output = Result < () , Si :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Pin :: new (& mut self . sink) . poll_close (cx) } }
};
}
