// Generated macro for ServerListening (struct)
macro_rules! Depcrate_events_connectivityServerListening {
() => {
// Module: crate::events::connectivity
// Provides: {"ServerListening"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] pub struct ServerListening { pub ip_v4 : Option < String > , pub ip_v6 : Option < String > , pub port_v4 : Option < u16 > , pub port_v6 : Option < u16 > , retry_required : Option < bool > , }
};
}
