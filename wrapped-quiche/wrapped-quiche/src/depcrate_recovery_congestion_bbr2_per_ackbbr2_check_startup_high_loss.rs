// Generated macro for bbr2_check_startup_high_loss (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_ackbbr2_check_startup_high_loss {
() => {
// Module: crate::recovery::congestion::bbr2::per_ack
// Provides: {"bbr2_check_startup_high_loss"}
// Dependencies: {}
fn bbr2_check_startup_high_loss (r : & mut Congestion) { if r . bbr2_state . loss_round_start && r . bbr2_state . in_recovery && r . bbr2_state . loss_events_in_round >= FULL_LOSS_COUNT as usize && per_loss :: bbr2_is_inflight_too_high (r) { bbr2_handle_queue_too_high_in_startup (r) ; } if r . bbr2_state . loss_round_start { r . bbr2_state . loss_events_in_round = 0 } }
};
}
