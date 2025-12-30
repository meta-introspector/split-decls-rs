// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl < B : Body > Body for http :: Request < B > { type Data = B :: Data ; type Error = B :: Error ; fn poll_frame (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < Frame < Self :: Data > , Self :: Error > > > { unsafe { self . map_unchecked_mut (http :: Request :: body_mut) . poll_frame (cx) } } fn is_end_stream (& self) -> bool { self . body () . is_end_stream () } fn size_hint (& self) -> SizeHint { self . body () . size_hint () } }
};
}
