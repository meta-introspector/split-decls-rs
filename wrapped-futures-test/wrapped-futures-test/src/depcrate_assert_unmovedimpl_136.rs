// Generated macro for impl_136 (impl)
macro_rules! Depcrate_assert_unmovedimpl_136 {
() => {
// Module: crate::assert_unmoved
// Provides: {"impl_136"}
// Dependencies: {}
impl < Si : Sink < Item > , Item > Sink < Item > for AssertUnmoved < Si > { type Error = Si :: Error ; fn poll_ready (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . poll_with (| s | s . poll_ready (cx)) } fn start_send (self : Pin < & mut Self > , item : Item) -> Result < () , Self :: Error > { self . poll_with (| s | s . start_send (item)) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . poll_with (| s | s . poll_flush (cx)) } fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . poll_with (| s | s . poll_close (cx)) } }
};
}
