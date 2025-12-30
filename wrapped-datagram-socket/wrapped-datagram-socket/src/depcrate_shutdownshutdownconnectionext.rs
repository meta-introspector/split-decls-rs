// Generated macro for ShutdownConnectionExt (trait)
macro_rules! Depcrate_shutdownShutdownConnectionExt {
() => {
// Module: crate::shutdown
// Provides: {"ShutdownConnectionExt"}
// Dependencies: {}
# [doc = " Shuts down a datagram oriented connection."] # [doc = ""] # [doc = " Implemented as an extension trait, adding utility methods to all"] # [doc = " [`ShutdownConnection`] types. Callers will tend to import this trait instead"] # [doc = " of [`ShutdownConnection`]."] # [doc = ""] # [doc = " [`ShutdownConnection`]: ShutdownConnection"] pub trait ShutdownConnectionExt : ShutdownConnection { # [inline] fn shutdown_connection (& mut self) -> impl Future < Output = io :: Result < () > > { poll_fn (move | cx | self . poll_shutdown (cx)) } }
};
}
