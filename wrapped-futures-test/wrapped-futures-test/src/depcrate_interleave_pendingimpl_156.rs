// Generated macro for impl_156 (impl)
macro_rules! Depcrate_interleave_pendingimpl_156 {
() => {
// Module: crate::interleave_pending
// Provides: {"impl_156"}
// Dependencies: {}
impl < Si : Sink < Item > , Item > Sink < Item > for InterleavePending < Si > { type Error = Si :: Error ; fn poll_ready (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . poll_with (cx , Si :: poll_ready) } fn start_send (self : Pin < & mut Self > , item : Item) -> Result < () , Self :: Error > { self . project () . inner . start_send (item) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . poll_with (cx , Si :: poll_flush) } fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . poll_with (cx , Si :: poll_close) } }
};
}
