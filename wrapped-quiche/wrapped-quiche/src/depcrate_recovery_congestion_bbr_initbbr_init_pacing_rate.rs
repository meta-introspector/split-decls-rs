// Generated macro for bbr_init_pacing_rate (function)
macro_rules! Depcrate_recovery_congestion_bbr_initbbr_init_pacing_rate {
() => {
// Module: crate::recovery::congestion::bbr::init
// Provides: {"bbr_init_pacing_rate"}
// Dependencies: {}
fn bbr_init_pacing_rate (r : & mut Congestion) { let bbr = & mut r . bbr_state ; let srtt = r . initial_rtt . as_secs_f64 () ; let nominal_bandwidth = r . congestion_window as f64 / srtt ; bbr . pacing_rate = (bbr . pacing_gain * nominal_bandwidth) as u64 ; }
};
}
