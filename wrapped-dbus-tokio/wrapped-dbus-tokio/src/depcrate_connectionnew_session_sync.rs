// Generated macro for new_session_sync (function)
macro_rules! Depcrate_connectionnew_session_sync {
() => {
// Module: crate::connection
// Provides: {"new_session_sync"}
// Dependencies: {}
# [doc = " Creates a connection to the session bus, to use with Tokio's default (multi-thread) scheduler."] # [doc = ""] # [doc = " Note: This function blocks until the connection is set up."] pub fn new_session_sync () -> Result < (IOResource < SyncConnection > , Arc < SyncConnection >) , dbus :: Error > { new (BusType :: Session) }
};
}
