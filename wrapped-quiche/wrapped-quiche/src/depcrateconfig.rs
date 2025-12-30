// Generated macro for Config (struct)
macro_rules! DepcrateConfig {
() => {
// Module: crate
// Provides: {"Config"}
// Dependencies: {}
# [doc = " Stores configuration shared between multiple connections."] pub struct Config { local_transport_params : TransportParams , version : u32 , tls_ctx : tls :: Context , application_protos : Vec < Vec < u8 > > , grease : bool , cc_algorithm : CongestionControlAlgorithm , custom_bbr_params : Option < BbrParams > , initial_congestion_window_packets : usize , enable_relaxed_loss_threshold : bool , pmtud : bool , hystart : bool , pacing : bool , # [doc = " Send rate limit in Mbps"] max_pacing_rate : Option < u64 > , tx_cap_factor : f64 , dgram_recv_max_queue_len : usize , dgram_send_max_queue_len : usize , path_challenge_recv_max_queue_len : usize , max_send_udp_payload_size : usize , max_connection_window : u64 , max_stream_window : u64 , max_amplification_factor : usize , disable_dcid_reuse : bool , track_unknown_transport_params : Option < usize > , initial_rtt : Duration , }
};
}
