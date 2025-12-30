// Generated macro for bbr2_init_pacing_rate (function)
macro_rules! Depcrate_recovery_congestion_bbr2_pacingbbr2_init_pacing_rate {
() => {
// Module: crate::recovery::congestion::bbr2::pacing
// Provides: {"bbr2_init_pacing_rate"}
// Dependencies: {}
pub fn bbr2_init_pacing_rate (r : & mut Congestion) { let bbr = & mut r . bbr2_state ; let srtt = r . initial_rtt . as_secs_f64 () ; let nominal_bandwidth = r . congestion_window as f64 / srtt ; bbr . pacing_rate = (STARTUP_PACING_GAIN * nominal_bandwidth) as u64 ; bbr . init_pacing_rate = (STARTUP_PACING_GAIN * nominal_bandwidth) as u64 ; }
};
}
