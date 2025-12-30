// Generated macro for congestion_event (function)
macro_rules! Depcrate_recovery_congestion_bbrcongestion_event {
() => {
// Module: crate::recovery::congestion::bbr
// Provides: {"congestion_event"}
// Dependencies: {}
fn congestion_event (r : & mut Congestion , bytes_in_flight : usize , lost_bytes : usize , largest_lost_pkt : & Sent , now : Instant ,) { r . bbr_state . newly_lost_bytes = lost_bytes ; if ! r . in_congestion_recovery (largest_lost_pkt . time_sent) { bbr_enter_recovery (r , bytes_in_flight - lost_bytes , now) ; } }
};
}
