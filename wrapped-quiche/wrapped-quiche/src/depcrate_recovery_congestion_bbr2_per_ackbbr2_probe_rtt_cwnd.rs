// Generated macro for bbr2_probe_rtt_cwnd (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_probe_rtt_cwnd {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_probe_rtt_cwnd"}
// Dependencies: {}
fn bbr2_probe_rtt_cwnd (r : & mut Congestion) -> usize { let probe_rtt_cwnd = bbr2_bdp_multiple (r , r . bbr2_state . bw , PROBE_RTT_CWND_GAIN) ; probe_rtt_cwnd . max (bbr2_min_pipe_cwnd (r)) }
};
}
