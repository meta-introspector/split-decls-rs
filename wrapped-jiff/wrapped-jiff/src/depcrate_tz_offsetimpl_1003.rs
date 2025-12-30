// Generated macro for impl_1003 (impl)
macro_rules! Depcrate_tz_offsetimpl_1003 {
() => {
// Module: crate::tz::offset
// Provides: {"impl_1003"}
// Dependencies: {}
# [doc = " Subtracts an unsigned duration of time from an offset. This panics on"] # [doc = " overflow."] # [doc = ""] # [doc = " For checked arithmetic, see [`Offset::checked_sub`]."] impl Sub < UnsignedDuration > for Offset { type Output = Offset ; # [inline] fn sub (self , rhs : UnsignedDuration) -> Offset { self . checked_sub (rhs) . expect ("subtracting unsigned duration from offsetsshould not overflow" ,) } }
};
}
