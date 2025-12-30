// Generated macro for impl_143 (impl)
macro_rules! Depcrate_future_future_remote_handleimpl_143 {
() => {
// Module: crate::future::future::remote_handle
// Provides: {"impl_143"}
// Dependencies: {}
impl < Fut : Future > Future for Remote < Fut > { type Output = () ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < () > { let this = self . project () ; if this . tx . as_mut () . unwrap () . poll_canceled (cx) . is_ready () && ! this . keep_running . load (Ordering :: SeqCst) { return Poll :: Ready (()) ; } let output = ready ! (this . future . poll (cx)) ; drop (this . tx . take () . unwrap () . send (output)) ; Poll :: Ready (()) } }
};
}
