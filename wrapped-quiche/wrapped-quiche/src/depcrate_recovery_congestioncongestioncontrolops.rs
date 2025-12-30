// Generated macro for CongestionControlOps (struct)
macro_rules! Depcrate_recovery_congestionCongestionControlOps {
() => {
// Module: crate::recovery::congestion
// Provides: {"CongestionControlOps"}
// Dependencies: {}
pub (crate) struct CongestionControlOps { pub on_init : fn (r : & mut Congestion) , pub on_packet_sent : fn (r : & mut Congestion , sent_bytes : usize , bytes_in_flight : usize , now : Instant ,) , pub on_packets_acked : fn (r : & mut Congestion , bytes_in_flight : usize , packets : & mut Vec < Acked > , now : Instant , rtt_stats : & RttStats ,) , pub congestion_event : fn (r : & mut Congestion , bytes_in_flight : usize , lost_bytes : usize , largest_lost_packet : & Sent , now : Instant ,) , pub checkpoint : fn (r : & mut Congestion) , pub rollback : fn (r : & mut Congestion) -> bool , pub has_custom_pacing : fn () -> bool , # [cfg (feature = "qlog")] pub state_str : fn (r : & Congestion , now : Instant) -> & 'static str , pub debug_fmt : fn (r : & Congestion , formatter : & mut std :: fmt :: Formatter ,) -> std :: fmt :: Result , }
};
}
