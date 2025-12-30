// Generated macro for State (struct)
macro_rules! Depcrate_recovery_congestion_bbr2State {
() => {
// Module: crate::recovery::congestion::bbr2
// Provides: {"State"}
// Dependencies: {}
# [doc = " BBR2 Specific State Variables."] pub struct State { tx_in_flight : usize , lost : usize , newly_acked_bytes : usize , newly_lost_bytes : usize , pacing_rate : u64 , init_pacing_rate : u64 , pacing_gain : f64 , cwnd_gain : f64 , packet_conservation : bool , state : BBR2StateMachine , round_count : u64 , round_start : bool , next_round_delivered : usize , idle_restart : bool , max_bw : u64 , bw_hi : u64 , bw_lo : u64 , bw : u64 , min_rtt : Duration , bdp : usize , extra_acked : usize , offload_budget : usize , max_inflight : usize , inflight_hi : usize , inflight_lo : usize , bw_latest : u64 , inflight_latest : usize , max_bw_filter : Minmax < u64 > , cycle_count : u64 , extra_acked_interval_start : Instant , extra_acked_delivered : usize , extra_acked_filter : Minmax < usize > , filled_pipe : bool , full_bw : u64 , full_bw_count : usize , min_rtt_stamp : Instant , probe_rtt_min_delay : Duration , probe_rtt_min_stamp : Instant , probe_rtt_expired : bool , in_recovery : bool , start_time : Instant , prior_cwnd : usize , bw_probe_samples : bool , probe_up_cnt : usize , prior_bytes_in_flight : usize , probe_rtt_done_stamp : Option < Instant > , probe_rtt_round_done : bool , bw_probe_wait : Duration , rounds_since_probe : usize , cycle_stamp : Instant , ack_phase : BBR2AckPhase , bw_probe_up_rounds : usize , bw_probe_up_acks : usize , loss_round_start : bool , loss_round_delivered : usize , loss_in_round : bool , loss_events_in_round : usize , }
};
}
