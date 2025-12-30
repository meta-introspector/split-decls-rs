// Generated macro for impl_186 (impl)
macro_rules! Depcrate_utilimpl_186 {
() => {
// Module: crate::util
// Provides: {"impl_186"}
// Dependencies: {}
impl NiceDuration { # [doc = " Returns the number of seconds in this duration in fraction form."] # [doc = " The number to the left of the decimal point is the number of seconds,"] # [doc = " and the number to the right is the number of milliseconds."] fn fractional_seconds (& self) -> f64 { let fractional = (self . 0 . subsec_nanos () as f64) / 1_000_000_000.0 ; self . 0 . as_secs () as f64 + fractional } }
};
}
