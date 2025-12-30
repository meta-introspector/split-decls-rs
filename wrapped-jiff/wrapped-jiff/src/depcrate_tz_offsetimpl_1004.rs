// Generated macro for impl_1004 (impl)
macro_rules! Depcrate_tz_offsetimpl_1004 {
() => {
// Module: crate::tz::offset
// Provides: {"impl_1004"}
// Dependencies: {}
# [doc = " Subtracts an unsigned duration of time from an offset in place. This panics"] # [doc = " on overflow."] # [doc = ""] # [doc = " For checked arithmetic, see [`Offset::checked_sub`]."] impl SubAssign < UnsignedDuration > for Offset { # [inline] fn sub_assign (& mut self , rhs : UnsignedDuration) { * self = self . sub (rhs) ; } }
};
}
