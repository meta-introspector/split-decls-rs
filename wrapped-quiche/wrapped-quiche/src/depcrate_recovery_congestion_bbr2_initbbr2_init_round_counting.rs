// Generated macro for bbr2_init_round_counting (function)
macro_rules! Depcrate_recovery_congestion_bbr2_initbbr2_init_round_counting {
() => {
// Module: crate::recovery::congestion::bbr2::init
// Provides: {"bbr2_init_round_counting"}
// Dependencies: {}
fn bbr2_init_round_counting (r : & mut Congestion) { let bbr = & mut r . bbr2_state ; bbr . next_round_delivered = 0 ; bbr . round_start = false ; bbr . round_count = 0 ; }
};
}
