// Generated macro for bbr_init_round_counting (function)
macro_rules! Depcrate_recovery_congestion_bbr_initbbr_init_round_counting {
() => {
// Module: crate::recovery::congestion::bbr::init
// Provides: {"bbr_init_round_counting"}
// Dependencies: {}
fn bbr_init_round_counting (r : & mut Congestion) { let bbr = & mut r . bbr_state ; bbr . next_round_delivered = 0 ; bbr . round_start = false ; bbr . round_count = 0 ; }
};
}
