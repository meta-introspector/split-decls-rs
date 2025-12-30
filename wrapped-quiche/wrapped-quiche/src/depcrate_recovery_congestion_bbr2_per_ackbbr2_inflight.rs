// Generated macro for bbr2_inflight (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_inflight {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_inflight"}
// Dependencies: {}
fn bbr2_inflight (r : & mut Congestion , bw : u64 , gain : f64) -> usize { let inflight = bbr2_bdp_multiple (r , bw , gain) ; bbr2_quantization_budget (r , inflight) }
};
}
