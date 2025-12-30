// Generated macro for new_system_local (function)
macro_rules! Depcrate_connectionnew_system_local {
() => {
// Module: crate::connection
// Provides: {"new_system_local"}
// Dependencies: {}
# [doc = " Creates a connection to the system bus, to use with Tokio's basic (single-thread) scheduler."] # [doc = ""] # [doc = " Note: This function blocks until the connection is set up."] pub fn new_system_local () -> Result < (IOResource < LocalConnection > , Arc < LocalConnection >) , dbus :: Error > { new (BusType :: System) }
};
}
