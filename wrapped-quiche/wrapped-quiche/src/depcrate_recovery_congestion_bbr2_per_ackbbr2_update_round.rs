// Generated macro for bbr2_update_round (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_update_round {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_update_round"}
// Dependencies: {}
fn bbr2_update_round (r : & mut Congestion , packet : & Acked) { if packet . delivered >= r . bbr2_state . next_round_delivered { bbr2_start_round (r) ; r . bbr2_state . round_count += 1 ; r . bbr2_state . rounds_since_probe += 1 ; r . bbr2_state . round_start = true ; } else { r . bbr2_state . round_start = false ; } }
};
}
