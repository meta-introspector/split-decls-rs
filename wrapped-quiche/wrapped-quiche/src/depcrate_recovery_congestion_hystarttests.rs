// Generated macro for tests (module)
macro_rules! Depcrate_recovery_congestion_hystarttests {
() => {
// Module: crate::recovery::congestion::hystart
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn start_round () { let mut hspp = Hystart :: default () ; let pkt_num = 100 ; hspp . start_round (pkt_num) ; assert_eq ! (hspp . window_end , Some (pkt_num)) ; assert_eq ! (hspp . current_round_min_rtt , Duration :: MAX) ; } # [test] fn css_cwnd_inc () { let hspp = Hystart :: default () ; let datagram_size = 1200 ; let css_cwnd_inc = hspp . css_cwnd_inc (datagram_size) ; assert_eq ! (datagram_size / CSS_GROWTH_DIVISOR , css_cwnd_inc) ; } # [test] fn congestion_event () { let mut hspp = Hystart :: default () ; let pkt_num = 100 ; hspp . start_round (pkt_num) ; assert_eq ! (hspp . window_end , Some (pkt_num)) ; hspp . congestion_event () ; assert_eq ! (hspp . window_end , None) ; } }
};
}
