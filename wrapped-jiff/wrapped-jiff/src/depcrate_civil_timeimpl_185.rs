// Generated macro for impl_185 (impl)
macro_rules! Depcrate_civil_timeimpl_185 {
() => {
// Module: crate::civil::time
// Provides: {"impl_185"}
// Dependencies: {}
# [doc = " Adds an unsigned duration of time. This uses wrapping arithmetic."] # [doc = ""] # [doc = " For checked arithmetic, see [`Time::checked_add`]."] impl core :: ops :: Add < UnsignedDuration > for Time { type Output = Time ; # [inline] fn add (self , rhs : UnsignedDuration) -> Time { self . wrapping_add (rhs) } }
};
}
