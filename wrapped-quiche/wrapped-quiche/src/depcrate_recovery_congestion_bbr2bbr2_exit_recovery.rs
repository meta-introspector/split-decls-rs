// Generated macro for bbr2_exit_recovery (function)
macro_rules! Depcrate_recovery_congestion_bbr2bbr2_exit_recovery {
() => {
// Module: crate::recovery::congestion::bbr2
// Provides: {"bbr2_exit_recovery"}
// Dependencies: {}
fn bbr2_exit_recovery (r : & mut Congestion) { r . congestion_recovery_start_time = None ; r . bbr2_state . packet_conservation = false ; r . bbr2_state . in_recovery = false ; per_ack :: bbr2_restore_cwnd (r) ; }
};
}
