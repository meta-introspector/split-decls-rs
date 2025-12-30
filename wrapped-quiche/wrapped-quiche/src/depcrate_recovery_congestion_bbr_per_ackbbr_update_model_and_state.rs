// Generated macro for bbr_update_model_and_state (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_ackbbr_update_model_and_state {
() => {
// Module: crate::recovery::congestion::bbr::per_ack
// Provides: {"bbr_update_model_and_state"}
// Dependencies: {}
pub fn bbr_update_model_and_state (r : & mut Congestion , packet : & Acked , bytes_in_flight : usize , now : Instant ,) { bbr_update_btlbw (r , packet , bytes_in_flight) ; bbr_check_cycle_phase (r , now) ; bbr_check_full_pipe (r) ; bbr_check_drain (r , bytes_in_flight , now) ; bbr_update_rtprop (r , now) ; bbr_check_probe_rtt (r , bytes_in_flight , now) ; }
};
}
