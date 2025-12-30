// Generated macro for bbr2_on_transmit (function)
macro_rules! Depcrate_recovery_congestion_bbr2_per_transmitbbr2_on_transmit {
() => {
// Module: crate::recovery::congestion::bbr2::per_transmit
// Provides: {"bbr2_on_transmit"}
// Dependencies: {}
pub fn bbr2_on_transmit (r : & mut Congestion , bytes_in_flight : usize , now : Instant ,) { bbr2_handle_restart_from_idle (r , bytes_in_flight , now) ; }
};
}
