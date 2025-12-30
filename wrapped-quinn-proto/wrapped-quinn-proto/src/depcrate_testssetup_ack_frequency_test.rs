// Generated macro for setup_ack_frequency_test (function)
macro_rules! Depcrate_testssetup_ack_frequency_test {
() => {
// Module: crate::tests
// Provides: {"setup_ack_frequency_test"}
// Dependencies: {}
fn setup_ack_frequency_test (max_ack_delay : Duration) -> (Pair , ConnectionHandle , ConnectionHandle) { let mut client_config = client_config_with_deterministic_pns () ; let mut ack_freq_config = AckFrequencyConfig :: default () ; ack_freq_config . ack_eliciting_threshold (10u32 . into ()) . max_ack_delay (Some (max_ack_delay)) ; Arc :: get_mut (& mut client_config . transport) . unwrap () . ack_frequency_config (Some (ack_freq_config)) . mtu_discovery_config (None) . initial_rtt (Duration :: from_millis (10)) ; let mut pair = Pair :: default_with_deterministic_pns () ; pair . latency = Duration :: from_millis (10) ; let (client_ch , server_ch) = pair . connect_with (client_config) ; pair . drive () ; assert_eq ! (pair . client_conn_mut (client_ch) . stats () . frame_tx . ack_frequency , 1) ; assert_eq ! (pair . client_conn_mut (client_ch) . stats () . frame_tx . ping , 0) ; (pair , client_ch , server_ch) }
};
}
