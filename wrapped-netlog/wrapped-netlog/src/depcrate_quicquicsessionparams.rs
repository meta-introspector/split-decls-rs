// Generated macro for QuicSessionParams (struct)
macro_rules! Depcrate_quicQuicSessionParams {
() => {
// Module: crate::quic
// Provides: {"QuicSessionParams"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Debug , Default)] pub struct QuicSessionParams { pub cert_verify_flags : u64 , pub connection_id : String , pub host : String , pub port : u16 , # [serde (alias = "network_anonymization_key")] pub network_isolation_key : String , pub privacy_mode : String , }
};
}
