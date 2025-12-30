// Generated macro for impl_725 (impl)
macro_rules! Depcrate_signed_durationimpl_725 {
() => {
// Module: crate::signed_duration
// Provides: {"impl_725"}
// Dependencies: {}
impl core :: ops :: Mul < i32 > for SignedDuration { type Output = SignedDuration ; # [inline] fn mul (self , rhs : i32) -> SignedDuration { self . checked_mul (rhs) . expect ("overflow when multiplying signed duration by scalar") } }
};
}
