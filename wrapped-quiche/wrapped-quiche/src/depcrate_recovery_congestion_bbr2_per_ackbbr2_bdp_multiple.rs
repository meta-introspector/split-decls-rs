// Generated macro for bbr2_bdp_multiple (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_bdp_multiple {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_bdp_multiple"}
// Dependencies: {}
fn bbr2_bdp_multiple (r : & mut Congestion , bw : u64 , gain : f64) -> usize { let bbr = & mut r . bbr2_state ; if bbr . min_rtt == Duration :: MAX { return r . max_datagram_size * r . initial_congestion_window_packets ; } bbr . bdp = (bw as f64 * bbr . min_rtt . as_secs_f64 ()) as usize ; (gain * bbr . bdp as f64) as usize }
};
}
