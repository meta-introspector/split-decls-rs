// Generated macro for impl_730 (impl)
macro_rules! Depcrate_signed_durationimpl_730 {
() => {
// Module: crate::signed_duration
// Provides: {"impl_730"}
// Dependencies: {}
impl core :: ops :: Div < i32 > for SignedDuration { type Output = SignedDuration ; # [inline] fn div (self , rhs : i32) -> SignedDuration { self . checked_div (rhs) . expect ("overflow when dividing signed duration by scalar") } }
};
}
