// Generated macro for impl_115 (impl)
macro_rules! Depcrate_emptyimpl_115 {
() => {
// Module: crate::empty
// Provides: {"impl_115"}
// Dependencies: {}
impl < D : Buf > Body for Empty < D > { type Data = D ; type Error = Infallible ; # [inline] fn poll_frame (self : Pin < & mut Self > , _cx : & mut Context < '_ > ,) -> Poll < Option < Result < Frame < Self :: Data > , Self :: Error > > > { Poll :: Ready (None) } fn is_end_stream (& self) -> bool { true } fn size_hint (& self) -> SizeHint { SizeHint :: with_exact (0) } }
};
}
