// Generated macro for impl_183 (impl)
macro_rules! Depcrate_civil_timeimpl_183 {
() => {
// Module: crate::civil::time
// Provides: {"impl_183"}
// Dependencies: {}
# [doc = " Subtracts a signed duration of time. This uses wrapping arithmetic."] # [doc = ""] # [doc = " For checked arithmetic, see [`Time::checked_sub`]."] impl core :: ops :: Sub < SignedDuration > for Time { type Output = Time ; # [inline] fn sub (self , rhs : SignedDuration) -> Time { self . wrapping_sub (rhs) } }
};
}
