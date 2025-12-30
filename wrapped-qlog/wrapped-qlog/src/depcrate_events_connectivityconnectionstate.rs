// Generated macro for ConnectionState (enum)
macro_rules! Depcrate_events_connectivityConnectionState {
() => {
// Module: crate::events::connectivity
// Provides: {"ConnectionState"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] # [serde (rename_all = "snake_case")] pub enum ConnectionState { Attempted , PeerValidated , HandshakeStarted , EarlyWrite , HandshakeCompleted , HandshakeConfirmed , Closing , Draining , Closed , }
};
}
