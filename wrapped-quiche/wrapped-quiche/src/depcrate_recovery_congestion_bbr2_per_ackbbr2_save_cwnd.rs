// Generated macro for bbr2_save_cwnd (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_save_cwnd {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_save_cwnd"}
// Dependencies: {}
pub fn bbr2_save_cwnd (r : & mut Congestion) -> usize { if ! r . bbr2_state . in_recovery && r . bbr2_state . state != BBR2StateMachine :: ProbeRTT { r . congestion_window } else { r . congestion_window . max (r . bbr2_state . prior_cwnd) } }
};
}
