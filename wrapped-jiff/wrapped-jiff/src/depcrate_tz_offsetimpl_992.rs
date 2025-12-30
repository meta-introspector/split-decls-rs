// Generated macro for impl_992 (impl)
macro_rules! Depcrate_tz_offsetimpl_992 {
() => {
// Module: crate::tz::offset
// Provides: {"impl_992"}
// Dependencies: {}
# [doc = " Adds a span of time to an offset. This panics on overflow."] # [doc = ""] # [doc = " For checked arithmetic, see [`Offset::checked_add`]."] impl Add < Span > for Offset { type Output = Offset ; # [inline] fn add (self , rhs : Span) -> Offset { self . checked_add (rhs) . expect ("adding span to offset should not overflow") } }
};
}
