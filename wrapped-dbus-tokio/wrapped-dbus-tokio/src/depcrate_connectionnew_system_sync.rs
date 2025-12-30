// Generated macro for new_system_sync (function)
macro_rules! Depcrate_connectionnew_system_sync {
() => {
// Module: crate::connection
// Provides: {"new_system_sync"}
// Dependencies: {}
# [doc = " Creates a connection to the system bus, to use with Tokio's default (multi-thread) scheduler."] # [doc = ""] # [doc = " Note: This function blocks until the connection is set up."] pub fn new_system_sync () -> Result < (IOResource < SyncConnection > , Arc < SyncConnection >) , dbus :: Error > { new (BusType :: System) }
};
}
