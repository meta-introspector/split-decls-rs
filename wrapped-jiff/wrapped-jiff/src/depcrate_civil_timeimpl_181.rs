// Generated macro for impl_181 (impl)
macro_rules! Depcrate_civil_timeimpl_181 {
() => {
// Module: crate::civil::time
// Provides: {"impl_181"}
// Dependencies: {}
# [doc = " Adds a signed duration of time. This uses wrapping arithmetic."] # [doc = ""] # [doc = " For checked arithmetic, see [`Time::checked_add`]."] impl core :: ops :: Add < SignedDuration > for Time { type Output = Time ; # [inline] fn add (self , rhs : SignedDuration) -> Time { self . wrapping_add (rhs) } }
};
}
