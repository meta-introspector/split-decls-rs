// Generated macro for VersionInformation (struct)
macro_rules! Depcrate_events_quicVersionInformation {
() => {
// Module: crate::events::quic
// Provides: {"VersionInformation"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug , Default)] pub struct VersionInformation { pub server_versions : Option < Vec < Bytes > > , pub client_versions : Option < Vec < Bytes > > , pub chosen_version : Option < Bytes > , }
};
}
