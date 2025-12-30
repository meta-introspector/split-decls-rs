// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl < T : Body + Unpin + ? Sized > Body for Box < T > { type Data = T :: Data ; type Error = T :: Error ; fn poll_frame (mut self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < Frame < Self :: Data > , Self :: Error > > > { Pin :: new (& mut * * self) . poll_frame (cx) } fn is_end_stream (& self) -> bool { self . as_ref () . is_end_stream () } fn size_hint (& self) -> SizeHint { self . as_ref () . size_hint () } }
};
}
