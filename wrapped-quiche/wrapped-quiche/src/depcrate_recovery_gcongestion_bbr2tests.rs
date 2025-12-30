// Generated macro for tests (module)
macro_rules! Depcrate_recovery_gcongestion_bbr2tests {
() => {
// Module: crate::recovery::gcongestion::bbr2
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use rstest :: rstest ; use super :: * ; # [rstest] fn update_mss (# [values (false , true)] scale_pacing_rate_by_mss : bool) { const INIT_PACKET_SIZE : usize = 1200 ; const INIT_WINDOW_PACKETS : usize = 10 ; const MAX_WINDOW_PACKETS : usize = 10000 ; const INIT_CWND : usize = INIT_WINDOW_PACKETS * INIT_PACKET_SIZE ; const MAX_CWND : usize = MAX_WINDOW_PACKETS * INIT_PACKET_SIZE ; let initial_rtt = Duration :: from_millis (333) ; let bbr_params = & BbrParams { scale_pacing_rate_by_mss : Some (scale_pacing_rate_by_mss) , .. Default :: default () } ; const NEW_PACKET_SIZE : usize = 1450 ; const NEW_CWND : usize = INIT_WINDOW_PACKETS * NEW_PACKET_SIZE ; const NEW_MAX_CWND : usize = MAX_WINDOW_PACKETS * NEW_PACKET_SIZE ; let mut bbr2 = BBRv2 :: new (INIT_WINDOW_PACKETS , MAX_WINDOW_PACKETS , INIT_PACKET_SIZE , initial_rtt , Some (bbr_params) ,) ; assert_eq ! (bbr2 . cwnd_limits . lo , INIT_CWND) ; assert_eq ! (bbr2 . cwnd_limits . hi , MAX_CWND) ; assert_eq ! (bbr2 . cwnd , INIT_CWND) ; assert_eq ! (bbr2 . pacing_rate . to_bytes_per_period (initial_rtt) , (2.88499 * INIT_CWND as f64) as u64) ; bbr2 . update_mss (NEW_PACKET_SIZE) ; assert_eq ! (bbr2 . cwnd_limits . lo , NEW_CWND) ; assert_eq ! (bbr2 . cwnd_limits . hi , NEW_MAX_CWND) ; assert_eq ! (bbr2 . cwnd , NEW_CWND) ; let pacing_cwnd = if scale_pacing_rate_by_mss { NEW_CWND } else { INIT_CWND } ; assert_eq ! (bbr2 . pacing_rate . to_bytes_per_period (initial_rtt) , (2.88499 * pacing_cwnd as f64) as u64) ; } }
};
}
