// Generated macro for Congestion (struct)
macro_rules! Depcrate_recovery_congestionCongestion {
() => {
// Module: crate::recovery::congestion
// Provides: {"Congestion"}
// Dependencies: {}
pub struct Congestion { pub (crate) cc_ops : & 'static CongestionControlOps , cubic_state : cubic :: State , pub (crate) hystart : hystart :: Hystart , pub (crate) pacer : pacer :: Pacer , pub (crate) prr : prr :: PRR , send_quantum : usize , bbr_state : bbr :: State , bbr2_state : bbr2 :: State , pub (crate) congestion_window : usize , pub (crate) ssthresh : SsThresh , bytes_acked_sl : usize , bytes_acked_ca : usize , pub (crate) congestion_recovery_start_time : Option < Instant > , pub (crate) app_limited : bool , pub (crate) delivery_rate : delivery_rate :: Rate , initial_rtt : Duration , # [doc = " Initial congestion window size in terms of packet count."] pub (crate) initial_congestion_window_packets : usize , max_datagram_size : usize , pub (crate) lost_count : usize , }
};
}
