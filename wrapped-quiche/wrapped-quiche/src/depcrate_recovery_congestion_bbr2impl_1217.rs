// Generated macro for impl_1217 (impl)
macro_rules! Depcrate_recovery_congestion_bbr2impl_1217 {
() => {
// Module: crate::recovery::congestion::bbr2
// Provides: {"impl_1217"}
// Dependencies: {}
impl State { pub fn new () -> Self { let now = Instant :: now () ; State { tx_in_flight : 0 , lost : 0 , newly_acked_bytes : 0 , newly_lost_bytes : 0 , pacing_rate : 0 , init_pacing_rate : 0 , pacing_gain : 0.0 , cwnd_gain : 0.0 , packet_conservation : false , state : BBR2StateMachine :: Startup , round_count : 0 , round_start : false , next_round_delivered : 0 , idle_restart : false , max_bw : 0 , bw_hi : u64 :: MAX , bw_lo : u64 :: MAX , bw : 0 , min_rtt : Duration :: MAX , bdp : 0 , extra_acked : 0 , offload_budget : 0 , max_inflight : 0 , inflight_hi : usize :: MAX , inflight_lo : usize :: MAX , bw_latest : 0 , inflight_latest : 0 , max_bw_filter : Minmax :: new (0) , cycle_count : 0 , extra_acked_interval_start : now , extra_acked_delivered : 0 , extra_acked_filter : Minmax :: new (0) , filled_pipe : false , full_bw : 0 , full_bw_count : 0 , min_rtt_stamp : now , probe_rtt_min_delay : Duration :: MAX , probe_rtt_min_stamp : now , probe_rtt_expired : false , in_recovery : false , start_time : now , prior_cwnd : 0 , bw_probe_samples : false , probe_up_cnt : 0 , prior_bytes_in_flight : 0 , probe_rtt_done_stamp : None , probe_rtt_round_done : false , bw_probe_wait : Duration :: ZERO , rounds_since_probe : 0 , cycle_stamp : now , ack_phase : BBR2AckPhase :: Init , bw_probe_up_rounds : 0 , bw_probe_up_acks : 0 , loss_round_start : false , loss_round_delivered : 0 , loss_in_round : false , loss_events_in_round : 0 , } } }
};
}
