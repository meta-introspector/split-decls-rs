// Generated macro for impl_723 (impl)
macro_rules! Depcrate_signed_durationimpl_723 {
() => {
// Module: crate::signed_duration
// Provides: {"impl_723"}
// Dependencies: {}
impl core :: ops :: Sub for SignedDuration { type Output = SignedDuration ; # [inline] fn sub (self , rhs : SignedDuration) -> SignedDuration { self . checked_sub (rhs) . expect ("overflow when subtracting signed durations") } }
};
}
