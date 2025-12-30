// Generated macro for impl_211 (impl)
macro_rules! Depcrate_future_try_future_into_futureimpl_211 {
() => {
// Module: crate::future::try_future::into_future
// Provides: {"impl_211"}
// Dependencies: {}
impl < Fut : TryFuture > Future for IntoFuture < Fut > { type Output = Result < Fut :: Ok , Fut :: Error > ; # [inline] fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . project () . future . try_poll (cx) } }
};
}
