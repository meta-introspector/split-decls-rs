// Generated macro for bbr_min_pipe_cwnd (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_ackbbr_min_pipe_cwnd {
() => {
// Module: crate::recovery::congestion::bbr::per_ack
// Provides: {"bbr_min_pipe_cwnd"}
// Dependencies: {}
# [doc = " The minimal cwnd value BBR tries to target, in bytes"] # [inline] fn bbr_min_pipe_cwnd (r : & mut Congestion) -> usize { BBR_MIN_PIPE_CWND_PKTS * r . max_datagram_size }
};
}
