// Generated macro for bbr_is_next_cycle_phase (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_ackbbr_is_next_cycle_phase {
() => {
// Module: crate::recovery::congestion::bbr::per_ack
// Provides: {"bbr_is_next_cycle_phase"}
// Dependencies: {}
fn bbr_is_next_cycle_phase (r : & mut Congestion , now : Instant) -> bool { let bbr = & mut r . bbr_state ; let lost_bytes = bbr . newly_lost_bytes ; let pacing_gain = bbr . pacing_gain ; let is_full_length = (now - bbr . cycle_stamp) > bbr . rtprop ; let prior_in_flight = bbr . prior_bytes_in_flight ; let prior_in_flight = bbr_bytes_in_net (r , prior_in_flight , now) ; if (pacing_gain - 1.0) . abs () < f64 :: EPSILON { return is_full_length ; } if pacing_gain > 1.0 { return is_full_length && (lost_bytes > 0 || prior_in_flight >= bbr_inflight (r , pacing_gain)) ; } is_full_length || prior_in_flight <= bbr_inflight (r , 1.0) }
};
}
