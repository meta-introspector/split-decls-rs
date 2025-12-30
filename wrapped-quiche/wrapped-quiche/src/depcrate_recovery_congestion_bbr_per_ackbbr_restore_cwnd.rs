// Generated macro for bbr_restore_cwnd (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_ackbbr_restore_cwnd {
() => {
// Module: crate::recovery::congestion::bbr::per_ack
// Provides: {"bbr_restore_cwnd"}
// Dependencies: {}
pub fn bbr_restore_cwnd (r : & mut Congestion) { r . congestion_window = r . congestion_window . max (r . bbr_state . prior_cwnd) ; }
};
}
