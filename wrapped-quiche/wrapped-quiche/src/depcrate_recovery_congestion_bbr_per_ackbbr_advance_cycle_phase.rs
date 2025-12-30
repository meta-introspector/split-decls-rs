// Generated macro for bbr_advance_cycle_phase (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_ackbbr_advance_cycle_phase {
() => {
// Module: crate::recovery::congestion::bbr::per_ack
// Provides: {"bbr_advance_cycle_phase"}
// Dependencies: {}
fn bbr_advance_cycle_phase (r : & mut Congestion , now : Instant) { let bbr = & mut r . bbr_state ; bbr . cycle_stamp = now ; bbr . cycle_index = (bbr . cycle_index + 1) % BBR_GAIN_CYCLE_LEN ; bbr . pacing_gain = PACING_GAIN_CYCLE [bbr . cycle_index] ; }
};
}
