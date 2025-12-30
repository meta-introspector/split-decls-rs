// Generated macro for bbr2_init_full_pipe (function)
macro_rules! Depcrate_recovery_congestion_bbr2_initbbr2_init_full_pipe {
() => {
// Module: crate::recovery::congestion::bbr2::init
// Provides: {"bbr2_init_full_pipe"}
// Dependencies: {}
fn bbr2_init_full_pipe (r : & mut Congestion) { let bbr = & mut r . bbr2_state ; bbr . filled_pipe = false ; bbr . full_bw = 0 ; bbr . full_bw_count = 0 ; }
};
}
