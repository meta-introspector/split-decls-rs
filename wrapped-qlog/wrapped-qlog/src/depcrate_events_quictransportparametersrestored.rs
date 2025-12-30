// Generated macro for TransportParametersRestored (struct)
macro_rules! Depcrate_events_quicTransportParametersRestored {
() => {
// Module: crate::events::quic
// Provides: {"TransportParametersRestored"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug , Default)] pub struct TransportParametersRestored { pub disable_active_migration : Option < bool > , pub max_idle_timeout : Option < u64 > , pub max_udp_payload_size : Option < u32 > , pub active_connection_id_limit : Option < u32 > , pub initial_max_data : Option < u64 > , pub initial_max_stream_data_bidi_local : Option < u64 > , pub initial_max_stream_data_bidi_remote : Option < u64 > , pub initial_max_stream_data_uni : Option < u64 > , pub initial_max_streams_bidi : Option < u64 > , pub initial_max_streams_uni : Option < u64 > , }
};
}
