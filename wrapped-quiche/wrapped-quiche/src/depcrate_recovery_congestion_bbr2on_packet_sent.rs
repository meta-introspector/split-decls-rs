// Generated macro for on_packet_sent (function)
macro_rules! Depcrate_recovery_congestion_bbr2on_packet_sent {
() => {
// Module: crate::recovery::congestion::bbr2
// Provides: {"on_packet_sent"}
// Dependencies: {}
fn on_packet_sent (r : & mut Congestion , _sent_bytes : usize , bytes_in_flight : usize , now : Instant ,) { per_transmit :: bbr2_on_transmit (r , bytes_in_flight , now) ; }
};
}
