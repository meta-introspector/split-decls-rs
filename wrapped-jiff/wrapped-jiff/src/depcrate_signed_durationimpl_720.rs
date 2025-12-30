// Generated macro for impl_720 (impl)
macro_rules! Depcrate_signed_durationimpl_720 {
() => {
// Module: crate::signed_duration
// Provides: {"impl_720"}
// Dependencies: {}
impl core :: ops :: Neg for SignedDuration { type Output = SignedDuration ; # [inline] fn neg (self) -> SignedDuration { self . checked_neg () . expect ("overflow when negating signed duration") } }
};
}
