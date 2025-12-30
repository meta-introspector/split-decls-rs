// Generated macro for impl_336 (impl)
macro_rules! Depcrate_fut_futureimpl_336 {
() => {
// Module: crate::fut::future
// Provides: {"impl_336"}
// Dependencies: {}
impl < F , A > ActorFuture < A > for FutureWrap < F , A > where F : Future , A : Actor , { type Output = F :: Output ; fn poll (self : Pin < & mut Self > , _ : & mut A , _ : & mut A :: Context , task : & mut Context < '_ > ,) -> Poll < Self :: Output > { self . project () . fut . poll (task) } }
};
}
