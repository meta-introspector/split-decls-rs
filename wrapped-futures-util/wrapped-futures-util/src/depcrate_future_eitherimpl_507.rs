// Generated macro for impl_507 (impl)
macro_rules! Depcrate_future_eitherimpl_507 {
() => {
// Module: crate::future::either
// Provides: {"impl_507"}
// Dependencies: {}
impl < A , B > FusedStream for Either < A , B > where A : FusedStream , B : FusedStream < Item = A :: Item > , { fn is_terminated (& self) -> bool { match self { Self :: Left (x) => x . is_terminated () , Self :: Right (x) => x . is_terminated () , } } }
};
}
