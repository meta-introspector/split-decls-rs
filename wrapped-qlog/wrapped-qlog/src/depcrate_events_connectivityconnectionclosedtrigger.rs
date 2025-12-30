// Generated macro for ConnectionClosedTrigger (enum)
macro_rules! Depcrate_events_connectivityConnectionClosedTrigger {
() => {
// Module: crate::events::connectivity
// Provides: {"ConnectionClosedTrigger"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , Copy , PartialEq , Eq , Debug)] # [serde (rename_all = "snake_case")] pub enum ConnectionClosedTrigger { Clean , HandshakeTimeout , IdleTimeout , Error , StatelessReset , VersionMismatch , Application , }
};
}
