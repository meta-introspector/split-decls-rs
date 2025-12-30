// Generated macro for impl_997 (impl)
macro_rules! Depcrate_tz_offsetimpl_997 {
() => {
// Module: crate::tz::offset
// Provides: {"impl_997"}
// Dependencies: {}
# [doc = " Adds a signed duration of time to an offset. This panics on overflow."] # [doc = ""] # [doc = " For checked arithmetic, see [`Offset::checked_add`]."] impl Add < SignedDuration > for Offset { type Output = Offset ; # [inline] fn add (self , rhs : SignedDuration) -> Offset { self . checked_add (rhs) . expect ("adding signed duration to offset should not overflow") } }
};
}
