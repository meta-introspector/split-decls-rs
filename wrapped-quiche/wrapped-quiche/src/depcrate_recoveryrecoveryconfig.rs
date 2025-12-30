// Generated macro for RecoveryConfig (struct)
macro_rules! Depcrate_recoveryRecoveryConfig {
() => {
// Module: crate::recovery
// Provides: {"RecoveryConfig"}
// Dependencies: {}
# [derive (Clone , Copy , PartialEq)] pub struct RecoveryConfig { pub initial_rtt : Duration , pub max_send_udp_payload_size : usize , pub max_ack_delay : Duration , pub cc_algorithm : CongestionControlAlgorithm , pub custom_bbr_params : Option < BbrParams > , pub hystart : bool , pub pacing : bool , pub max_pacing_rate : Option < u64 > , pub initial_congestion_window_packets : usize , pub enable_relaxed_loss_threshold : bool , }
};
}
