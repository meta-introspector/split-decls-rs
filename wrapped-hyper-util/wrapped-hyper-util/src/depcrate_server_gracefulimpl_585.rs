// Generated macro for impl_585 (impl)
macro_rules! Depcrate_server_gracefulimpl_585 {
() => {
// Module: crate::server::graceful
// Provides: {"impl_585"}
// Dependencies: {}
impl < C , F > Future for GracefulConnectionFuture < C , F > where C : GracefulConnection , F : Future , { type Output = C :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut task :: Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; if this . cancelled_guard . is_none () { if let Poll :: Ready (guard) = this . cancel . poll (cx) { this . cancelled_guard . set (Some (guard)) ; this . conn . as_mut () . graceful_shutdown () ; } } this . conn . poll (cx) } }
};
}
