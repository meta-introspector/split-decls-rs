// Generated macro for on_packet_sent (function)
macro_rules! Depcrate_recovery_congestion_cubicon_packet_sent {
() => {
// Module: crate::recovery::congestion::cubic
// Provides: {"on_packet_sent"}
// Dependencies: {}
fn on_packet_sent (r : & mut Congestion , sent_bytes : usize , bytes_in_flight : usize , now : Instant ,) { let cubic = & mut r . cubic_state ; if let Some (last_sent_time) = cubic . last_sent_time { if bytes_in_flight == 0 { let delta = now - last_sent_time ; if let Some (recovery_start_time) = r . congestion_recovery_start_time { if delta . as_nanos () > 0 { r . congestion_recovery_start_time = Some (recovery_start_time + delta) ; } } } } cubic . last_sent_time = Some (now) ; reno :: on_packet_sent (r , sent_bytes , bytes_in_flight , now) ; }
};
}
