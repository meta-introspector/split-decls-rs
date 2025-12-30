// Generated macro for impl_722 (impl)
macro_rules! Depcrate_future_wait_untilimpl_722 {
() => {
// Module: crate::future::wait_until
// Provides: {"impl_722"}
// Dependencies: {}
impl < F : Future , D : Future > Future for WaitUntil < F , D > { type Output = F :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; loop { match this . state { State :: Started => { ready ! (this . deadline . as_mut () . poll (cx)) ; * this . state = State :: PollFuture ; } State :: PollFuture => { let value = ready ! (this . future . as_mut () . poll (cx)) ; * this . state = State :: Completed ; return Poll :: Ready (value) ; } State :: Completed => panic ! ("future polled after completing") , } } } }
};
}
