// Generated macro for on_packet_acked (function)
macro_rules! Depcrate_recovery_congestion_renoon_packet_acked {
() => {
// Module: crate::recovery::congestion::reno
// Provides: {"on_packet_acked"}
// Dependencies: {}
fn on_packet_acked (r : & mut Congestion , packet : & Acked , now : Instant , rtt_stats : & RttStats ,) { if r . in_congestion_recovery (packet . time_sent) { return ; } if r . app_limited { return ; } if r . congestion_window < r . ssthresh . get () { r . bytes_acked_sl += packet . size ; if r . hystart . in_css () { r . congestion_window += r . hystart . css_cwnd_inc (r . max_datagram_size) ; } else { r . congestion_window += r . max_datagram_size ; } if r . hystart . on_packet_acked (packet , rtt_stats . latest_rtt , now) { r . ssthresh . update (r . congestion_window , true) ; } } else { r . bytes_acked_ca += packet . size ; if r . bytes_acked_ca >= r . congestion_window { r . bytes_acked_ca -= r . congestion_window ; r . congestion_window += r . max_datagram_size ; } } }
};
}
