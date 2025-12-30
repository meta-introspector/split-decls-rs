// Generated macro for impl_187 (impl)
macro_rules! Depcrate_civil_timeimpl_187 {
() => {
// Module: crate::civil::time
// Provides: {"impl_187"}
// Dependencies: {}
# [doc = " Subtracts an unsigned duration of time. This uses wrapping arithmetic."] # [doc = ""] # [doc = " For checked arithmetic, see [`Time::checked_sub`]."] impl core :: ops :: Sub < UnsignedDuration > for Time { type Output = Time ; # [inline] fn sub (self , rhs : UnsignedDuration) -> Time { self . wrapping_sub (rhs) } }
};
}
