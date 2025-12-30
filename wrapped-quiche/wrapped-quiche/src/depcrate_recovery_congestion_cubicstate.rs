// Generated macro for State (struct)
macro_rules! Depcrate_recovery_congestion_cubicState {
() => {
// Module: crate::recovery::congestion::cubic
// Provides: {"State"}
// Dependencies: {}
# [doc = " CUBIC State Variables."] # [doc = ""] # [doc = " We need to keep those variables across the connection."] # [doc = " k, w_max, w_est are described in the RFC."] # [derive (Debug , Default)] pub struct State { k : f64 , w_max : f64 , w_est : f64 , alpha_aimd : f64 , last_sent_time : Option < Instant > , cwnd_inc : usize , prior : PriorState , }
};
}
