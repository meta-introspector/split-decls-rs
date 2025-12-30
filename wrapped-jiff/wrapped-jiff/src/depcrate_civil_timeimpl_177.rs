// Generated macro for impl_177 (impl)
macro_rules! Depcrate_civil_timeimpl_177 {
() => {
// Module: crate::civil::time
// Provides: {"impl_177"}
// Dependencies: {}
# [doc = " Adds a span of time in place. This uses wrapping arithmetic."] # [doc = ""] # [doc = " For checked arithmetic, see [`Time::checked_add`]."] impl core :: ops :: AddAssign < Span > for Time { # [inline] fn add_assign (& mut self , rhs : Span) { * self = * self + rhs ; } }
};
}
