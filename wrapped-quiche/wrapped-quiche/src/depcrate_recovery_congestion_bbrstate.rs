// Generated macro for State (struct)
macro_rules! Depcrate_recovery_congestion_bbrState {
() => {
// Module: crate::recovery::congestion::bbr
// Provides: {"State"}
// Dependencies: {}
# [doc = " BBR Specific State Variables."] pub struct State { state : BBRStateMachine , pacing_rate : u64 , btlbw : u64 , btlbwfilter : Minmax < u64 > , rtprop : Duration , rtprop_stamp : Instant , rtprop_expired : bool , pacing_gain : f64 , cwnd_gain : f64 , filled_pipe : bool , round_count : u64 , round_start : bool , next_round_delivered : usize , probe_rtt_done_stamp : Option < Instant > , probe_rtt_round_done : bool , packet_conservation : bool , prior_cwnd : usize , idle_restart : bool , full_bw : u64 , full_bw_count : usize , cycle_stamp : Instant , cycle_index : usize , target_cwnd : usize , in_recovery : bool , start_time : Instant , newly_lost_bytes : usize , newly_acked_bytes : usize , prior_bytes_in_flight : usize , }
};
}
