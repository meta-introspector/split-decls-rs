// Generated macro for bbr2_set_send_quantum (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_set_send_quantum {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_set_send_quantum"}
// Dependencies: {}
fn bbr2_set_send_quantum (r : & mut Congestion) { let bbr = & mut r . bbr2_state ; let rate = bbr . pacing_rate ; let floor = if rate < PACING_RATE_1_2MBPS { r . max_datagram_size } else { 2 * r . max_datagram_size } ; r . send_quantum = cmp :: min ((rate / 1000_u64) as usize , 64 * 1024) ; r . send_quantum = r . send_quantum . max (floor) ; }
};
}
