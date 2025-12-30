// Generated macro for ConnectionIdUpdated (struct)
macro_rules! Depcrate_events_connectivityConnectionIdUpdated {
() => {
// Module: crate::events::connectivity
// Provides: {"ConnectionIdUpdated"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] pub struct ConnectionIdUpdated { pub owner : Option < TransportOwner > , pub old : Option < Bytes > , pub new : Option < Bytes > , }
};
}
