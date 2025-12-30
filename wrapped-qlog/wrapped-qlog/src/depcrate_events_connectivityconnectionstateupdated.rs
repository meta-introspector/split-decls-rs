// Generated macro for ConnectionStateUpdated (struct)
macro_rules! Depcrate_events_connectivityConnectionStateUpdated {
() => {
// Module: crate::events::connectivity
// Provides: {"ConnectionStateUpdated"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] pub struct ConnectionStateUpdated { pub old : Option < ConnectionState > , pub new : ConnectionState , }
};
}
