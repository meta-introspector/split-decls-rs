// Generated macro for impl_993 (impl)
macro_rules! Depcrate_tz_offsetimpl_993 {
() => {
// Module: crate::tz::offset
// Provides: {"impl_993"}
// Dependencies: {}
# [doc = " Adds a span of time to an offset in place. This panics on overflow."] # [doc = ""] # [doc = " For checked arithmetic, see [`Offset::checked_add`]."] impl AddAssign < Span > for Offset { # [inline] fn add_assign (& mut self , rhs : Span) { * self = self . add (rhs) ; } }
};
}
