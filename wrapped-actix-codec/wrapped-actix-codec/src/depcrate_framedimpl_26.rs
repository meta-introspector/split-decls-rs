// Generated macro for impl_26 (impl)
macro_rules! Depcrate_framedimpl_26 {
() => {
// Module: crate::framed
// Provides: {"impl_26"}
// Dependencies: {}
impl < T , U , I > Sink < I > for Framed < T , U > where T : AsyncWrite , U : Encoder < I > , U :: Error : From < io :: Error > , { type Error = U :: Error ; fn poll_ready (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { if self . is_write_ready () { Poll :: Ready (Ok (())) } else { self . flush (cx) } } fn start_send (self : Pin < & mut Self > , item : I) -> Result < () , Self :: Error > { self . write (item) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . flush (cx) } fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . close (cx) } }
};
}
