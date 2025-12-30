// Generated macro for impl_577 (impl)
macro_rules! Depcrate_server_gracefulimpl_577 {
() => {
// Module: crate::server::graceful
// Provides: {"impl_577"}
// Dependencies: {}
impl GracefulShutdown { # [doc = " Create a new graceful shutdown helper."] pub fn new () -> Self { let (tx , _) = watch :: channel (()) ; Self { tx } } # [doc = " Wrap a future for graceful shutdown watching."] pub fn watch < C : GracefulConnection > (& self , conn : C) -> impl Future < Output = C :: Output > { self . watcher () . watch (conn) } # [doc = " Create an owned type that can watch a connection."] # [doc = ""] # [doc = " This method allows created an owned type that can be sent onto another"] # [doc = " task before calling [`Watcher::watch()`]."] pub fn watcher (& self) -> Watcher { let rx = self . tx . subscribe () ; Watcher { rx } } # [doc = " Signal shutdown for all watched connections."] # [doc = ""] # [doc = " This returns a `Future` which will complete once all watched"] # [doc = " connections have shutdown."] pub async fn shutdown (self) { let Self { tx } = self ; let _ = tx . send (()) ; tx . closed () . await ; } # [doc = " Returns the number of the watching connections."] pub fn count (& self) -> usize { self . tx . receiver_count () } }
};
}
