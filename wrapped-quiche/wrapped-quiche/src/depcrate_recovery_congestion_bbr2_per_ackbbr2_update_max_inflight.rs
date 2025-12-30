// Generated macro for bbr2_update_max_inflight (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_update_max_inflight {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_update_max_inflight"}
// Dependencies: {}
fn bbr2_update_max_inflight (r : & mut Congestion) { let inflight = bbr2_bdp_multiple (r , r . bbr2_state . max_bw , r . bbr2_state . cwnd_gain) ; let inflight = inflight + r . bbr2_state . extra_acked ; r . bbr2_state . max_inflight = bbr2_quantization_budget (r , inflight) ; }
};
}
