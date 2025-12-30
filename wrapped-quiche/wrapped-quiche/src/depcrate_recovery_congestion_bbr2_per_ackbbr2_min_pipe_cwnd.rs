// Generated macro for bbr2_min_pipe_cwnd (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_min_pipe_cwnd {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_min_pipe_cwnd"}
// Dependencies: {}
# [doc = " The minimal cwnd value BBR2 tries to target, in bytes"] # [inline] fn bbr2_min_pipe_cwnd (r : & mut Congestion) -> usize { MIN_PIPE_CWND_PKTS * r . max_datagram_size }
};
}
