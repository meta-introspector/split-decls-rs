// Generated macro for impl_506 (impl)
macro_rules! Depcrate_future_eitherimpl_506 {
() => {
// Module: crate::future::either
// Provides: {"impl_506"}
// Dependencies: {}
impl < A , B > Stream for Either < A , B > where A : Stream , B : Stream < Item = A :: Item > , { type Item = A :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { match self . as_pin_mut () { Either :: Left (x) => x . poll_next (cx) , Either :: Right (x) => x . poll_next (cx) , } } fn size_hint (& self) -> (usize , Option < usize >) { match self { Self :: Left (x) => x . size_hint () , Self :: Right (x) => x . size_hint () , } } }
};
}
