// Generated macro for impl_721 (impl)
macro_rules! Depcrate_signed_durationimpl_721 {
() => {
// Module: crate::signed_duration
// Provides: {"impl_721"}
// Dependencies: {}
impl core :: ops :: Add for SignedDuration { type Output = SignedDuration ; # [inline] fn add (self , rhs : SignedDuration) -> SignedDuration { self . checked_add (rhs) . expect ("overflow when adding signed durations") } }
};
}
