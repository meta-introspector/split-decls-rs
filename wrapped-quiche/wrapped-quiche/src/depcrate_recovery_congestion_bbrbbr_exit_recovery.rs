// Generated macro for bbr_exit_recovery (function)
macro_rules! Depcrate_recovery_congestion_bbrbbr_exit_recovery {
() => {
// Module: crate::recovery::congestion::bbr
// Provides: {"bbr_exit_recovery"}
// Dependencies: {}
fn bbr_exit_recovery (r : & mut Congestion) { r . congestion_recovery_start_time = None ; r . bbr_state . packet_conservation = false ; r . bbr_state . in_recovery = false ; per_ack :: bbr_restore_cwnd (r) ; }
};
}
