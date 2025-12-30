// Generated macro for impl_1892 (impl)
macro_rules! Depcrate_sink_flushimpl_1892 {
() => {
// Module: crate::sink::flush
// Provides: {"impl_1892"}
// Dependencies: {}
impl < Si : Sink < Item > + Unpin + ? Sized , Item > Future for Flush < '_ , Si , Item > { type Output = Result < () , Si :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Pin :: new (& mut self . sink) . poll_flush (cx) } }
};
}
