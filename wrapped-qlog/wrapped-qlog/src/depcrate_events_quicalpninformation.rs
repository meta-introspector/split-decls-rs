// Generated macro for AlpnInformation (struct)
macro_rules! Depcrate_events_quicAlpnInformation {
() => {
// Module: crate::events::quic
// Provides: {"AlpnInformation"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug , Default)] pub struct AlpnInformation { pub server_alpns : Option < Vec < Bytes > > , pub client_alpns : Option < Vec < Bytes > > , pub chosen_alpn : Option < Bytes > , }
};
}
