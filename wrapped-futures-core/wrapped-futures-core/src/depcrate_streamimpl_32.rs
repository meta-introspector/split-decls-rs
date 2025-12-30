// Generated macro for impl_32 (impl)
macro_rules! Depcrate_streamimpl_32 {
() => {
// Module: crate::stream
// Provides: {"impl_32"}
// Dependencies: {}
impl < S , T , E > TryStream for S where S : ? Sized + Stream < Item = Result < T , E > > , { type Ok = T ; type Error = E ; fn try_poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < Self :: Ok , Self :: Error > > > { self . poll_next (cx) } }
};
}
