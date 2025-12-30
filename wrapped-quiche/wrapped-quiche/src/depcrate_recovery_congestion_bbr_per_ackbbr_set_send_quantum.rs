// Generated macro for bbr_set_send_quantum (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_ackbbr_set_send_quantum {
() => {
// Module: crate::recovery::congestion::bbr::per_ack
// Provides: {"bbr_set_send_quantum"}
// Dependencies: {}
fn bbr_set_send_quantum (r : & mut Congestion) { let rate = r . bbr_state . pacing_rate ; r . send_quantum = match rate { rate if rate < PACING_RATE_1_2MBPS => r . max_datagram_size , rate if rate < PACING_RATE_24MBPS => 2 * r . max_datagram_size , _ => cmp :: min ((rate / 1000_u64) as usize , 64 * 1024) , } }
};
}
