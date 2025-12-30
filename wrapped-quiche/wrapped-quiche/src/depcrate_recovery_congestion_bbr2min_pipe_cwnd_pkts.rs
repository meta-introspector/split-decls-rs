// Generated macro for MIN_PIPE_CWND_PKTS (const)
macro_rules! Depcrate_recovery_congestion_bbr2MIN_PIPE_CWND_PKTS {
() => {
// Module: crate::recovery::congestion::bbr2
// Provides: {"MIN_PIPE_CWND_PKTS"}
// Dependencies: {}
# [doc = " The minimal cwnd value BBR targets, to allow"] # [doc = " pipelining with TCP endpoints that follow an \"ACK every other packet\""] # [doc = " delayed-ACK policy: 4 * SMSS."] const MIN_PIPE_CWND_PKTS : usize = 4 ;
};
}
