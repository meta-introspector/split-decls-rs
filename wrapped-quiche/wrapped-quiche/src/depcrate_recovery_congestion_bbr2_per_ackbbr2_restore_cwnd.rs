// Generated macro for bbr2_restore_cwnd (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_restore_cwnd {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_restore_cwnd"}
// Dependencies: {}
pub fn bbr2_restore_cwnd (r : & mut Congestion) { r . congestion_window = r . congestion_window . max (r . bbr2_state . prior_cwnd) ; }
};
}
