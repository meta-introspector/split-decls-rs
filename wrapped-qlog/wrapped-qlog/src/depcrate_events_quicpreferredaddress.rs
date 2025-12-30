// Generated macro for PreferredAddress (struct)
macro_rules! Depcrate_events_quicPreferredAddress {
() => {
// Module: crate::events::quic
// Provides: {"PreferredAddress"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] pub struct PreferredAddress { pub ip_v4 : String , pub ip_v6 : String , pub port_v4 : u16 , pub port_v6 : u16 , pub connection_id : Bytes , pub stateless_reset_token : StatelessResetToken , }
};
}
