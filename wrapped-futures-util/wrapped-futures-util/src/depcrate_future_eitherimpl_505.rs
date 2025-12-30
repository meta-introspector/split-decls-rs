// Generated macro for impl_505 (impl)
macro_rules! Depcrate_future_eitherimpl_505 {
() => {
// Module: crate::future::either
// Provides: {"impl_505"}
// Dependencies: {}
impl < A , B > FusedFuture for Either < A , B > where A : FusedFuture , B : FusedFuture < Output = A :: Output > , { fn is_terminated (& self) -> bool { match self { Self :: Left (x) => x . is_terminated () , Self :: Right (x) => x . is_terminated () , } } }
};
}
