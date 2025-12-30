// Generated macro for impl_1001 (impl)
macro_rules! Depcrate_tz_offsetimpl_1001 {
() => {
// Module: crate::tz::offset
// Provides: {"impl_1001"}
// Dependencies: {}
# [doc = " Adds an unsigned duration of time to an offset. This panics on overflow."] # [doc = ""] # [doc = " For checked arithmetic, see [`Offset::checked_add`]."] impl Add < UnsignedDuration > for Offset { type Output = Offset ; # [inline] fn add (self , rhs : UnsignedDuration) -> Offset { self . checked_add (rhs) . expect ("adding unsigned duration to offset should not overflow") } }
};
}
