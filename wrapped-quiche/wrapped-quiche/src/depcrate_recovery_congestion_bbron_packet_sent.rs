// Generated macro for on_packet_sent (function)
macro_rules! Depcrate_recovery_congestion_bbron_packet_sent {
() => {
// Module: crate::recovery::congestion::bbr
// Provides: {"on_packet_sent"}
// Dependencies: {}
fn on_packet_sent (r : & mut Congestion , _sent_bytes : usize , bytes_in_flight : usize , _now : Instant ,) { per_transmit :: bbr_on_transmit (r , bytes_in_flight) ; }
};
}
