// Generated macro for impl_1341 (impl)
macro_rules! Depcrate_recovery_congestion_cubicimpl_1341 {
() => {
// Module: crate::recovery::congestion::cubic
// Provides: {"impl_1341"}
// Dependencies: {}
# [doc = " CUBIC Functions."] # [doc = ""] # [doc = " Note that these calculations are based on a count of cwnd as bytes,"] # [doc = " not packets."] # [doc = " Unit of t (duration) and RTT are based on seconds (f64)."] impl State { fn cubic_k (& self , cwnd : usize , max_datagram_size : usize) -> f64 { let w_max = self . w_max / max_datagram_size as f64 ; let cwnd = cwnd as f64 / max_datagram_size as f64 ; libm :: cbrt ((w_max - cwnd) / C) } fn w_cubic (& self , t : Duration , max_datagram_size : usize) -> f64 { let w_max = self . w_max / max_datagram_size as f64 ; (C * (t . as_secs_f64 () - self . k) . powi (3) + w_max) * max_datagram_size as f64 } fn w_est_inc (& self , acked : usize , cwnd : usize , max_datagram_size : usize ,) -> f64 { self . alpha_aimd * (acked as f64 / cwnd as f64) * max_datagram_size as f64 } }
};
}
