// Generated macro for impl_167 (impl)
macro_rules! Depcrate_streamimpl_167 {
() => {
// Module: crate::stream
// Provides: {"impl_167"}
// Dependencies: {}
impl < B > Body for BodyStream < B > where B : Body , { type Data = B :: Data ; type Error = B :: Error ; fn poll_frame (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < Frame < Self :: Data > , Self :: Error > > > { self . project () . body . poll_frame (cx) } }
};
}
