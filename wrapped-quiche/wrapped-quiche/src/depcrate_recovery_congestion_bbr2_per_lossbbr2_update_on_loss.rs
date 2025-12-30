// Generated macro for bbr2_update_on_loss (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_lossbbr2_update_on_loss {
() => {
// Module: crate::recovery::congestion::bbr2::per_loss
// Provides: {"bbr2_update_on_loss"}
// Dependencies: {}
pub fn bbr2_update_on_loss (r : & mut Congestion , packet : & Sent , lost_bytes : usize , now : Instant ,) { bbr2_handle_lost_packet (r , packet , lost_bytes , now) ; }
};
}
