// Generated macro for ConnectionClosed (struct)
macro_rules! Depcrate_events_connectivityConnectionClosed {
() => {
// Module: crate::events::connectivity
// Provides: {"ConnectionClosed"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] pub struct ConnectionClosed { pub owner : Option < TransportOwner > , pub connection_code : Option < ConnectionErrorCode > , pub application_code : Option < ApplicationErrorCode > , pub internal_code : Option < u32 > , pub reason : Option < String > , pub trigger : Option < ConnectionClosedTrigger > , }
};
}
