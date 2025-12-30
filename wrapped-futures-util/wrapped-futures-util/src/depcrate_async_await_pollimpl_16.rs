// Generated macro for impl_16 (impl)
macro_rules! Depcrate_async_await_pollimpl_16 {
() => {
// Module: crate::async_await::poll
// Provides: {"impl_16"}
// Dependencies: {}
impl < F : Future + Unpin > Future for PollOnce < F > { type Output = Poll < F :: Output > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Poll :: Ready (self . future . poll_unpin (cx)) } }
};
}
