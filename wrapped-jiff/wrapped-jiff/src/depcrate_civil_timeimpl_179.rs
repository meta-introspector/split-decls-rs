// Generated macro for impl_179 (impl)
macro_rules! Depcrate_civil_timeimpl_179 {
() => {
// Module: crate::civil::time
// Provides: {"impl_179"}
// Dependencies: {}
# [doc = " Subtracts a span of time in place. This uses wrapping arithmetic."] # [doc = ""] # [doc = " For checked arithmetic, see [`Time::checked_sub`]."] impl core :: ops :: SubAssign < Span > for Time { # [inline] fn sub_assign (& mut self , rhs : Span) { * self = * self - rhs ; } }
};
}
