// Generated macro for bbr_inflight (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_ackbbr_inflight {
() => {
// Module: crate::recovery::congestion::bbr::per_ack
// Provides: {"bbr_inflight"}
// Dependencies: {}
fn bbr_inflight (r : & mut Congestion , gain : f64) -> usize { let bbr = & mut r . bbr_state ; if bbr . rtprop == Duration :: MAX { return r . max_datagram_size * r . initial_congestion_window_packets ; } let quanta = 3 * r . send_quantum ; let estimated_bdp = bbr . btlbw as f64 * bbr . rtprop . as_secs_f64 () ; (gain * estimated_bdp) as usize + quanta }
};
}
