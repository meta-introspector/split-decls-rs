// Generated macro for impl_19 (impl)
macro_rules! Depcrate_parallel_async_executorimpl_19 {
() => {
// Module: crate::parallel::async_executor
// Provides: {"impl_19"}
// Dependencies: {}
impl Future for YieldOnce { type Output = () ; fn poll (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < () > { let flag = & mut std :: pin :: Pin :: into_inner (self) . 0 ; if ! * flag { * flag = true ; Poll :: Pending } else { Poll :: Ready (()) } } }
};
}
