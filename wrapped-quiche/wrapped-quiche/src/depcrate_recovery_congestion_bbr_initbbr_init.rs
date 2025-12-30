// Generated macro for bbr_init (function)
macro_rules! Depcrate_recovery_congestion_bbr_initbbr_init {
() => {
// Module: crate::recovery::congestion::bbr::init
// Provides: {"bbr_init"}
// Dependencies: {}
pub fn bbr_init (r : & mut Congestion) { let bbr = & mut r . bbr_state ; bbr . rtprop = r . initial_rtt ; bbr . rtprop_stamp = Instant :: now () ; bbr . next_round_delivered = r . delivery_rate . delivered () ; r . send_quantum = r . max_datagram_size ; bbr_init_round_counting (r) ; bbr_init_full_pipe (r) ; bbr_init_pacing_rate (r) ; bbr_enter_startup (r) ; }
};
}
