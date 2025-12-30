// Generated macro for impl_176 (impl)
macro_rules! Depcrate_civil_timeimpl_176 {
() => {
// Module: crate::civil::time
// Provides: {"impl_176"}
// Dependencies: {}
# [doc = " Adds a span of time. This uses wrapping arithmetic."] # [doc = ""] # [doc = " For checked arithmetic, see [`Time::checked_add`]."] impl core :: ops :: Add < Span > for Time { type Output = Time ; # [inline] fn add (self , rhs : Span) -> Time { self . wrapping_add (rhs) } }
};
}
