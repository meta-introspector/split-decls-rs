// Generated macro for impl_23 (impl)
macro_rules! Depcrate_combinators_box_bodyimpl_23 {
() => {
// Module: crate::combinators::box_body
// Provides: {"impl_23"}
// Dependencies: {}
impl < D , E > Body for BoxBody < D , E > where D : Buf , { type Data = D ; type Error = E ; fn poll_frame (mut self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < Frame < Self :: Data > , Self :: Error > > > { self . inner . as_mut () . poll_frame (cx) } fn is_end_stream (& self) -> bool { self . inner . is_end_stream () } fn size_hint (& self) -> SizeHint { self . inner . size_hint () } }
};
}
