// Generated macro for impl_635 (impl)
macro_rules! Depcrate_config_transportimpl_635 {
() => {
// Module: crate::config::transport
// Provides: {"impl_635"}
// Dependencies: {}
impl Default for TransportConfig { fn default () -> Self { const EXPECTED_RTT : u32 = 100 ; const MAX_STREAM_BANDWIDTH : u32 = 12500 * 1000 ; const STREAM_RWND : u32 = MAX_STREAM_BANDWIDTH / 1000 * EXPECTED_RTT ; Self { max_concurrent_bidi_streams : 100u32 . into () , max_concurrent_uni_streams : 100u32 . into () , max_idle_timeout : Some (VarInt (30_000)) , stream_receive_window : STREAM_RWND . into () , receive_window : VarInt :: MAX , send_window : (8 * STREAM_RWND) . into () , send_fairness : true , packet_threshold : 3 , time_threshold : 9.0 / 8.0 , initial_rtt : Duration :: from_millis (333) , initial_mtu : INITIAL_MTU , min_mtu : INITIAL_MTU , mtu_discovery_config : Some (MtuDiscoveryConfig :: default ()) , pad_to_mtu : false , ack_frequency_config : None , persistent_congestion_threshold : 3 , keep_alive_interval : None , crypto_buffer_size : 16 * 1024 , allow_spin : true , datagram_receive_buffer_size : Some (STREAM_RWND as usize) , datagram_send_buffer_size : 1024 * 1024 , # [cfg (test)] deterministic_packet_numbers : false , congestion_controller_factory : Arc :: new (congestion :: CubicConfig :: default ()) , enable_segmentation_offload : true , qlog_sink : QlogSink :: default () , } } }
};
}
