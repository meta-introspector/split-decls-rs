// Generated macro for impl_994 (impl)
macro_rules! Depcrate_tz_offsetimpl_994 {
() => {
// Module: crate::tz::offset
// Provides: {"impl_994"}
// Dependencies: {}
# [doc = " Subtracts a span of time from an offset. This panics on overflow."] # [doc = ""] # [doc = " For checked arithmetic, see [`Offset::checked_sub`]."] impl Sub < Span > for Offset { type Output = Offset ; # [inline] fn sub (self , rhs : Span) -> Offset { self . checked_sub (rhs) . expect ("subtracting span from offsetsshould not overflow") } }
};
}
