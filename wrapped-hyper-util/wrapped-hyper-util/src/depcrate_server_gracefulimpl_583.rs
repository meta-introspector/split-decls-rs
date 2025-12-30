// Generated macro for impl_583 (impl)
macro_rules! Depcrate_server_gracefulimpl_583 {
() => {
// Module: crate::server::graceful
// Provides: {"impl_583"}
// Dependencies: {}
impl < C , F : Future > GracefulConnectionFuture < C , F > { fn new (conn : C , cancel : F) -> Self { Self { conn , cancel , cancelled_guard : None , } } }
};
}
