// Generated macro for debug_fmt (function)
macro_rules! Depcrate_recovery_congestion_bbrdebug_fmt {
() => {
// Module: crate::recovery::congestion::bbr
// Provides: {"debug_fmt"}
// Dependencies: {}
fn debug_fmt (r : & Congestion , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { let bbr = & r . bbr_state ; write ! (f , "bbr={{ state={:?} btlbw={} rtprop={:?} pacing_rate={} pacing_gain={} cwnd_gain={} target_cwnd={} send_quantum={} filled_pipe={} round_count={} }}" , bbr . state , bbr . btlbw , bbr . rtprop , bbr . pacing_rate , bbr . pacing_gain , bbr . cwnd_gain , bbr . target_cwnd , r . send_quantum () , bbr . filled_pipe , bbr . round_count) }
};
}
