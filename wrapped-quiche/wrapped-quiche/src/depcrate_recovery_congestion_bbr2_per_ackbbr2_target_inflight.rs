// Generated macro for bbr2_target_inflight (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_target_inflight {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_target_inflight"}
// Dependencies: {}
pub fn bbr2_target_inflight (r : & mut Congestion) -> usize { r . bbr2_state . bdp . min (r . congestion_window) }
};
}
