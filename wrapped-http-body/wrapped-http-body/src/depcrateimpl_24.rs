// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl < B : Body > Body for http :: Response < B > { type Data = B :: Data ; type Error = B :: Error ; fn poll_frame (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < Frame < Self :: Data > , Self :: Error > > > { unsafe { self . map_unchecked_mut (http :: Response :: body_mut) . poll_frame (cx) } } fn is_end_stream (& self) -> bool { self . body () . is_end_stream () } fn size_hint (& self) -> SizeHint { self . body () . size_hint () } }
};
}
