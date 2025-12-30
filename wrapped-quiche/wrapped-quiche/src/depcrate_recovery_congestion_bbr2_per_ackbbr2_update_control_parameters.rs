// Generated macro for bbr2_update_control_parameters (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_update_control_parameters {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_update_control_parameters"}
// Dependencies: {}
pub fn bbr2_update_control_parameters (r : & mut Congestion , in_flight : usize , now : Instant ,) { pacing :: bbr2_set_pacing_rate (r) ; bbr2_set_send_quantum (r) ; r . set_pacing_rate (r . bbr2_state . pacing_rate , now) ; bbr2_set_cwnd (r , in_flight) ; }
};
}
