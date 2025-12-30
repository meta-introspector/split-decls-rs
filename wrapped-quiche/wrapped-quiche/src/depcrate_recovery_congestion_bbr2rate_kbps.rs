// Generated macro for rate_kbps (function)
macro_rules! Depcrate_recovery_congestion_bbr2rate_kbps {
() => {
// Module: crate::recovery::congestion::bbr2
// Provides: {"rate_kbps"}
// Dependencies: {}
fn rate_kbps (rate : u64) -> isize { if rate == u64 :: MAX { - 1 } else { (rate * 8 / 1000) as isize } }
};
}
