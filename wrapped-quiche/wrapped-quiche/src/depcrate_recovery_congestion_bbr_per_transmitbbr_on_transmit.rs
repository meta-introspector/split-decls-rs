// Generated macro for bbr_on_transmit (function)
macro_rules! Depcrate_recovery_congestion_bbr_per_transmitbbr_on_transmit {
() => {
// Module: crate::recovery::congestion::bbr::per_transmit
// Provides: {"bbr_on_transmit"}
// Dependencies: {}
pub fn bbr_on_transmit (r : & mut Congestion , bytes_in_flight : usize) { bbr_handle_restart_from_idle (r , bytes_in_flight) ; }
};
}
