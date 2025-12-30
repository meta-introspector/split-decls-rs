// Generated macro for TransportParametersSet (struct)
macro_rules! Depcrate_events_quicTransportParametersSet {
() => {
// Module: crate::events::quic
// Provides: {"TransportParametersSet"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug , Default)] pub struct TransportParametersSet { pub owner : Option < TransportOwner > , pub resumption_allowed : Option < bool > , pub early_data_enabled : Option < bool > , pub tls_cipher : Option < String > , pub aead_tag_length : Option < u8 > , pub original_destination_connection_id : Option < Bytes > , pub initial_source_connection_id : Option < Bytes > , pub retry_source_connection_id : Option < Bytes > , pub stateless_reset_token : Option < StatelessResetToken > , pub disable_active_migration : Option < bool > , pub max_idle_timeout : Option < u64 > , pub max_udp_payload_size : Option < u32 > , pub ack_delay_exponent : Option < u16 > , pub max_ack_delay : Option < u16 > , pub active_connection_id_limit : Option < u32 > , pub initial_max_data : Option < u64 > , pub initial_max_stream_data_bidi_local : Option < u64 > , pub initial_max_stream_data_bidi_remote : Option < u64 > , pub initial_max_stream_data_uni : Option < u64 > , pub initial_max_streams_bidi : Option < u64 > , pub initial_max_streams_uni : Option < u64 > , pub preferred_address : Option < PreferredAddress > , pub unknown_parameters : Vec < UnknownTransportParameter > , }
};
}
