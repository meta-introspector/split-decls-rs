// Generated macro for impl_178 (impl)
macro_rules! Depcrate_civil_timeimpl_178 {
() => {
// Module: crate::civil::time
// Provides: {"impl_178"}
// Dependencies: {}
# [doc = " Subtracts a span of time. This uses wrapping arithmetic."] # [doc = ""] # [doc = " For checked arithmetic, see [`Time::checked_sub`]."] impl core :: ops :: Sub < Span > for Time { type Output = Time ; # [inline] fn sub (self , rhs : Span) -> Time { self . wrapping_sub (rhs) } }
};
}
