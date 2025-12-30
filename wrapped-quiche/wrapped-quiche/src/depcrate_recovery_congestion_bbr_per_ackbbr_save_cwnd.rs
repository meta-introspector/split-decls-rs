// Generated macro for bbr_save_cwnd (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_ackbbr_save_cwnd {
() => {
// Module: crate::recovery::congestion::bbr::per_ack
// Provides: {"bbr_save_cwnd"}
// Dependencies: {}
pub fn bbr_save_cwnd (r : & mut Congestion) -> usize { if ! r . bbr_state . in_recovery && r . bbr_state . state != BBRStateMachine :: ProbeRTT { r . congestion_window } else { r . congestion_window . max (r . bbr_state . prior_cwnd) } }
};
}
