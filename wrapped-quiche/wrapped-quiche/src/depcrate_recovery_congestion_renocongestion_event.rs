// Generated macro for congestion_event (function)
macro_rules! Depcrate_recovery_congestion_renocongestion_event {
() => {
// Module: crate::recovery::congestion::reno
// Provides: {"congestion_event"}
// Dependencies: {}
fn congestion_event (r : & mut Congestion , _bytes_in_flight : usize , _lost_bytes : usize , largest_lost_pkt : & Sent , now : Instant ,) { let time_sent = largest_lost_pkt . time_sent ; if ! r . in_congestion_recovery (time_sent) { r . congestion_recovery_start_time = Some (now) ; r . congestion_window = (r . congestion_window as f64 * LOSS_REDUCTION_FACTOR) as usize ; r . congestion_window = cmp :: max (r . congestion_window , r . max_datagram_size * MINIMUM_WINDOW_PACKETS ,) ; r . bytes_acked_ca = (r . congestion_window as f64 * LOSS_REDUCTION_FACTOR) as usize ; r . ssthresh . update (r . congestion_window , r . hystart . in_css ()) ; if r . hystart . in_css () { r . hystart . congestion_event () ; } } }
};
}
