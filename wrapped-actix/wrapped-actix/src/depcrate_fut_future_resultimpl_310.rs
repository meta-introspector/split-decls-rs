// Generated macro for impl_310 (impl)
macro_rules! Depcrate_fut_future_resultimpl_310 {
() => {
// Module: crate::fut::future::result
// Provides: {"impl_310"}
// Dependencies: {}
impl < T , A > ActorFuture < A > for Ready < T > where A : Actor , { type Output = T ; # [inline] fn poll (self : Pin < & mut Self > , _ : & mut A , _ : & mut A :: Context , cx : & mut task :: Context < '_ > ,) -> Poll < Self :: Output > { Future :: poll (self , cx) } }
};
}
