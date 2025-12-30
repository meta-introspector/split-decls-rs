// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl < P > Body for Pin < P > where P : Unpin + ops :: DerefMut , P :: Target : Body , { type Data = < < P as ops :: Deref > :: Target as Body > :: Data ; type Error = < < P as ops :: Deref > :: Target as Body > :: Error ; fn poll_frame (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < Frame < Self :: Data > , Self :: Error > > > { Pin :: get_mut (self) . as_mut () . poll_frame (cx) } fn is_end_stream (& self) -> bool { self . as_ref () . is_end_stream () } fn size_hint (& self) -> SizeHint { self . as_ref () . size_hint () } }
};
}
