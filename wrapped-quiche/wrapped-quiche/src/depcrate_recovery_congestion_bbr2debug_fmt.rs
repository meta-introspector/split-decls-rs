// Generated macro for debug_fmt (function)
macro_rules! Depcrate_recovery_congestion_bbr2debug_fmt {
() => {
// Module: crate::recovery::congestion::bbr2
// Provides: {"debug_fmt"}
// Dependencies: {}
fn debug_fmt (r : & Congestion , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { let bbr = & r . bbr2_state ; write ! (f , "bbr2={{ ") ? ; write ! (f , "state={:?} in_recovery={} ack_phase={:?} filled_pipe={} full_bw_count={} loss_events_in_round={} " , bbr . state , bbr . in_recovery , bbr . ack_phase , bbr . filled_pipe , bbr . full_bw_count , bbr . loss_events_in_round) ? ; write ! (f , "send_quantum={} extra_acked={} min_rtt={:?} round_start={} " , r . send_quantum , bbr . extra_acked , bbr . min_rtt , bbr . round_start) ? ; write ! (f , "max_bw={}kbps bw_lo={}kbps bw={}kbps bw_hi={}kbps full_bw={}kbps " , rate_kbps (bbr . max_bw) , rate_kbps (bbr . bw_lo) , rate_kbps (bbr . bw) , rate_kbps (bbr . bw_hi) , rate_kbps (bbr . full_bw)) ? ; write ! (f , "inflight_lo={} inflight_hi={} max_inflight={} " , bbr . inflight_lo , bbr . inflight_hi , bbr . max_inflight) ? ; write ! (f , "probe_up_cnt={} bw_probe_samples={} " , bbr . probe_up_cnt , bbr . bw_probe_samples) ? ; write ! (f , "}}") }
};
}
