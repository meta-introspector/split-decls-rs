// Generated macro for impl_580 (impl)
macro_rules! Depcrate_server_gracefulimpl_580 {
() => {
// Module: crate::server::graceful
// Provides: {"impl_580"}
// Dependencies: {}
impl Watcher { # [doc = " Wrap a future for graceful shutdown watching."] pub fn watch < C : GracefulConnection > (self , conn : C) -> impl Future < Output = C :: Output > { let Watcher { mut rx } = self ; GracefulConnectionFuture :: new (conn , async move { let _ = rx . changed () . await ; rx }) } }
};
}
