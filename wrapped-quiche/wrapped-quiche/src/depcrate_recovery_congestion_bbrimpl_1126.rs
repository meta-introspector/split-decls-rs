// Generated macro for impl_1126 (impl)
macro_rules! Depcrate_recovery_congestion_bbrimpl_1126 {
() => {
// Module: crate::recovery::congestion::bbr
// Provides: {"impl_1126"}
// Dependencies: {}
impl State { pub fn new () -> Self { let now = Instant :: now () ; State { state : BBRStateMachine :: Startup , pacing_rate : 0 , btlbw : 0 , btlbwfilter : Minmax :: new (0) , rtprop : Duration :: ZERO , rtprop_stamp : now , rtprop_expired : false , pacing_gain : 0.0 , cwnd_gain : 0.0 , filled_pipe : false , round_count : 0 , round_start : false , next_round_delivered : 0 , probe_rtt_done_stamp : None , probe_rtt_round_done : false , packet_conservation : false , prior_cwnd : 0 , idle_restart : false , full_bw : 0 , full_bw_count : 0 , cycle_stamp : now , cycle_index : 0 , target_cwnd : 0 , in_recovery : false , start_time : now , newly_lost_bytes : 0 , newly_acked_bytes : 0 , prior_bytes_in_flight : 0 , } } }
};
}
