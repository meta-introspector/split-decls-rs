// Generated macro for congestion_event (function)
macro_rules! Depcrate_recovery_congestion_cubiccongestion_event {
() => {
// Module: crate::recovery::congestion::cubic
// Provides: {"congestion_event"}
// Dependencies: {}
fn congestion_event (r : & mut Congestion , bytes_in_flight : usize , _lost_bytes : usize , largest_lost_pkt : & Sent , now : Instant ,) { let time_sent = largest_lost_pkt . time_sent ; let in_congestion_recovery = r . in_congestion_recovery (time_sent) ; if ! in_congestion_recovery { r . congestion_recovery_start_time = Some (now) ; if (r . congestion_window as f64) < r . cubic_state . w_max { r . cubic_state . w_max = r . congestion_window as f64 * (1.0 + BETA_CUBIC) / 2.0 ; } else { r . cubic_state . w_max = r . congestion_window as f64 ; } let ssthresh = (r . congestion_window as f64 * BETA_CUBIC) as usize ; let ssthresh = cmp :: max (ssthresh , r . max_datagram_size * MINIMUM_WINDOW_PACKETS) ; r . ssthresh . update (ssthresh , r . hystart . in_css ()) ; r . congestion_window = ssthresh ; r . cubic_state . k = if r . cubic_state . w_max < r . congestion_window as f64 { 0.0 } else { r . cubic_state . cubic_k (r . congestion_window , r . max_datagram_size) } ; r . cubic_state . cwnd_inc = (r . cubic_state . cwnd_inc as f64 * BETA_CUBIC) as usize ; r . cubic_state . w_est = r . congestion_window as f64 ; r . cubic_state . alpha_aimd = ALPHA_AIMD ; if r . hystart . in_css () { r . hystart . congestion_event () ; } r . prr . congestion_event (bytes_in_flight) ; } }
};
}
