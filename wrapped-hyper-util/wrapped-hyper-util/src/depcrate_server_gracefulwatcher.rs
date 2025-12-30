// Generated macro for Watcher (struct)
macro_rules! Depcrate_server_gracefulWatcher {
() => {
// Module: crate::server::graceful
// Provides: {"Watcher"}
// Dependencies: {}
# [doc = " A watcher side of the graceful shutdown."] # [doc = ""] # [doc = " This type can only watch a connection, it cannot trigger a shutdown."] # [doc = ""] # [doc = " Call [`GracefulShutdown::watcher()`] to construct one of these."] pub struct Watcher { rx : watch :: Receiver < () > , }
};
}
