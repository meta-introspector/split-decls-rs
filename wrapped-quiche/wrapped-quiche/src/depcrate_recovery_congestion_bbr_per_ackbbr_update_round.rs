// Generated macro for bbr_update_round (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_ackbbr_update_round {
() => {
// Module: crate::recovery::congestion::bbr::per_ack
// Provides: {"bbr_update_round"}
// Dependencies: {}
fn bbr_update_round (r : & mut Congestion , packet : & Acked) { let bbr = & mut r . bbr_state ; if packet . delivered >= bbr . next_round_delivered { bbr . next_round_delivered = r . delivery_rate . delivered () ; bbr . round_count += 1 ; bbr . round_start = true ; bbr . packet_conservation = false ; } else { bbr . round_start = false ; } }
};
}
