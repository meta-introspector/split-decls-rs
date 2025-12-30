// Generated macro for bbr_update_control_parameters (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_ackbbr_update_control_parameters {
() => {
// Module: crate::recovery::congestion::bbr::per_ack
// Provides: {"bbr_update_control_parameters"}
// Dependencies: {}
pub fn bbr_update_control_parameters (r : & mut Congestion , bytes_in_flight : usize , now : Instant ,) { pacing :: bbr_set_pacing_rate (r) ; bbr_set_send_quantum (r) ; r . set_pacing_rate (r . bbr_state . pacing_rate , now) ; bbr_set_cwnd (r , bytes_in_flight) ; }
};
}
