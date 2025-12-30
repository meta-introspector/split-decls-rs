// Generated macro for GracefulConnection (trait)
macro_rules! Depcrate_server_gracefulGracefulConnection {
() => {
// Module: crate::server::graceful
// Provides: {"GracefulConnection"}
// Dependencies: {}
# [doc = " An internal utility trait as an umbrella target for all (hyper) connection"] # [doc = " types that the [`GracefulShutdown`] can watch."] pub trait GracefulConnection : Future < Output = Result < () , Self :: Error > > + private :: Sealed { # [doc = " The error type returned by the connection when used as a future."] type Error ; # [doc = " Start a graceful shutdown process for this connection."] fn graceful_shutdown (self : Pin < & mut Self >) ; }
};
}
