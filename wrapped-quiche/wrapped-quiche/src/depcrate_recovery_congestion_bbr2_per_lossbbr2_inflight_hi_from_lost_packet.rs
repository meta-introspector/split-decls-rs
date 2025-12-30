// Generated macro for bbr2_inflight_hi_from_lost_packet (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_lossbbr2_inflight_hi_from_lost_packet {
() => {
// Module: crate::recovery::congestion::bbr2::per_loss
// Provides: {"bbr2_inflight_hi_from_lost_packet"}
// Dependencies: {}
fn bbr2_inflight_hi_from_lost_packet (r : & mut Congestion , packet : & Sent) -> usize { let size = packet . size ; let inflight_prev = r . bbr2_state . tx_in_flight - size ; let lost_prev = r . bbr2_state . lost - size ; let lost_prefix = (LOSS_THRESH * inflight_prev as f64 - lost_prev as f64) / (1.0 - LOSS_THRESH) ; inflight_prev + lost_prefix as usize }
};
}
