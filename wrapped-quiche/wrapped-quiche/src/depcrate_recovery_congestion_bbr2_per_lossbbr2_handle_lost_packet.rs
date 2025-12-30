// Generated macro for bbr2_handle_lost_packet (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_lossbbr2_handle_lost_packet {
() => {
// Module: crate::recovery::congestion::bbr2::per_loss
// Provides: {"bbr2_handle_lost_packet"}
// Dependencies: {}
fn bbr2_handle_lost_packet (r : & mut Congestion , packet : & Sent , lost_bytes : usize , now : Instant ,) { if ! r . bbr2_state . bw_probe_samples { return ; } r . bbr2_state . tx_in_flight = packet . tx_in_flight ; r . bbr2_state . lost = lost_bytes ; r . delivery_rate . update_app_limited (packet . is_app_limited) ; if bbr2_is_inflight_too_high (r) { r . bbr2_state . tx_in_flight = bbr2_inflight_hi_from_lost_packet (r , packet) ; bbr2_handle_inflight_too_high (r , now) ; } }
};
}
