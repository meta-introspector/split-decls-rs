// Generated macro for TransportParameters (struct)
macro_rules! Depcrate_quicTransportParameters {
() => {
// Module: crate::quic
// Provides: {"TransportParameters"}
// Dependencies: {}
# [derive (Debug , Default)] pub struct TransportParameters { pub versions : String , pub max_idle_timeout : Option < u64 > , pub max_udp_payload_size : Option < u64 > , pub initial_max_data : Option < u64 > , pub initial_max_stream_data_bidi_local : Option < u64 > , pub initial_max_stream_data_bidi_remote : Option < u64 > , pub initial_max_stream_data_uni : Option < u64 > , pub initial_max_streams_bidi : Option < u64 > , pub initial_max_streams_uni : Option < u64 > , pub initial_source_connection_id : Option < String > , pub max_datagram_frame_size : Option < u64 > , }
};
}
