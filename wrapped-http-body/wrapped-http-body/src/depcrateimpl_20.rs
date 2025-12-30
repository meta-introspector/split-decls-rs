// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl < T : Body + Unpin + ? Sized > Body for & mut T { type Data = T :: Data ; type Error = T :: Error ; fn poll_frame (mut self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < Frame < Self :: Data > , Self :: Error > > > { Pin :: new (& mut * * self) . poll_frame (cx) } fn is_end_stream (& self) -> bool { Pin :: new (& * * self) . is_end_stream () } fn size_hint (& self) -> SizeHint { Pin :: new (& * * self) . size_hint () } }
};
}
