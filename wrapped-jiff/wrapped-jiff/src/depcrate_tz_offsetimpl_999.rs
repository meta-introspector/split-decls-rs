// Generated macro for impl_999 (impl)
macro_rules! Depcrate_tz_offsetimpl_999 {
() => {
// Module: crate::tz::offset
// Provides: {"impl_999"}
// Dependencies: {}
# [doc = " Subtracts a signed duration of time from an offset. This panics on"] # [doc = " overflow."] # [doc = ""] # [doc = " For checked arithmetic, see [`Offset::checked_sub`]."] impl Sub < SignedDuration > for Offset { type Output = Offset ; # [inline] fn sub (self , rhs : SignedDuration) -> Offset { self . checked_sub (rhs) . expect ("subtracting signed duration from offsetsshould not overflow" ,) } }
};
}
