// Generated macro for bbr_init_full_pipe (function)
macro_rules! Depcrate_recovery_congestion_bbr_initbbr_init_full_pipe {
() => {
// Module: crate::recovery::congestion::bbr::init
// Provides: {"bbr_init_full_pipe"}
// Dependencies: {}
fn bbr_init_full_pipe (r : & mut Congestion) { let bbr = & mut r . bbr_state ; bbr . filled_pipe = false ; bbr . full_bw = 0 ; bbr . full_bw_count = 0 ; }
};
}
