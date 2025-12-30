// Generated macro for impl_995 (impl)
macro_rules! Depcrate_tz_offsetimpl_995 {
() => {
// Module: crate::tz::offset
// Provides: {"impl_995"}
// Dependencies: {}
# [doc = " Subtracts a span of time from an offset in place. This panics on overflow."] # [doc = ""] # [doc = " For checked arithmetic, see [`Offset::checked_sub`]."] impl SubAssign < Span > for Offset { # [inline] fn sub_assign (& mut self , rhs : Span) { * self = self . sub (rhs) ; } }
};
}
