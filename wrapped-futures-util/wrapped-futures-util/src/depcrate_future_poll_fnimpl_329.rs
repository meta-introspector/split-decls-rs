// Generated macro for impl_329 (impl)
macro_rules! Depcrate_future_poll_fnimpl_329 {
() => {
// Module: crate::future::poll_fn
// Provides: {"impl_329"}
// Dependencies: {}
impl < T , F > Future for PollFn < F > where F : FnMut (& mut Context < '_ >) -> Poll < T > , { type Output = T ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < T > { (& mut self . f) (cx) } }
};
}
