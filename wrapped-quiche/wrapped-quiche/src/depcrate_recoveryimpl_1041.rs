// Generated macro for impl_1041 (impl)
macro_rules! Depcrate_recoveryimpl_1041 {
() => {
// Module: crate::recovery
// Provides: {"impl_1041"}
// Dependencies: {}
impl RecoveryConfig { pub fn from_config (config : & Config) -> Self { Self { initial_rtt : config . initial_rtt , max_send_udp_payload_size : config . max_send_udp_payload_size , max_ack_delay : Duration :: ZERO , cc_algorithm : config . cc_algorithm , custom_bbr_params : config . custom_bbr_params , hystart : config . hystart , pacing : config . pacing , max_pacing_rate : config . max_pacing_rate , initial_congestion_window_packets : config . initial_congestion_window_packets , enable_relaxed_loss_threshold : config . enable_relaxed_loss_threshold , } } }
};
}
