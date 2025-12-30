// Generated macro for bbr_update_target_cwnd (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_ackbbr_update_target_cwnd {
() => {
// Module: crate::recovery::congestion::bbr::per_ack
// Provides: {"bbr_update_target_cwnd"}
// Dependencies: {}
fn bbr_update_target_cwnd (r : & mut Congestion) { r . bbr_state . target_cwnd = bbr_inflight (r , r . bbr_state . cwnd_gain) ; }
};
}
