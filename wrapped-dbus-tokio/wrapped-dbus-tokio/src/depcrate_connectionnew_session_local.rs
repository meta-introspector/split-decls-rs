// Generated macro for new_session_local (function)
macro_rules! Depcrate_connectionnew_session_local {
() => {
// Module: crate::connection
// Provides: {"new_session_local"}
// Dependencies: {}
# [doc = " Creates a connection to the session bus, to use with Tokio's basic (single-thread) scheduler."] # [doc = ""] # [doc = " Note: This function blocks until the connection is set up."] pub fn new_session_local () -> Result < (IOResource < LocalConnection > , Arc < LocalConnection >) , dbus :: Error > { new (BusType :: Session) }
};
}
