// Generated macro for congestion_event (function)
macro_rules! Depcrate_recovery_congestion_bbr2congestion_event {
() => {
// Module: crate::recovery::congestion::bbr2
// Provides: {"congestion_event"}
// Dependencies: {}
fn congestion_event (r : & mut Congestion , bytes_in_flight : usize , lost_bytes : usize , largest_lost_pkt : & Sent , now : Instant ,) { r . bbr2_state . newly_lost_bytes = lost_bytes ; per_loss :: bbr2_update_on_loss (r , largest_lost_pkt , lost_bytes , now) ; if ! r . in_congestion_recovery (largest_lost_pkt . time_sent) { bbr2_enter_recovery (r , bytes_in_flight - lost_bytes , now) ; } }
};
}
