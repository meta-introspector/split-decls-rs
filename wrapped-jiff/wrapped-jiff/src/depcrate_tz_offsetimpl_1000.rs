// Generated macro for impl_1000 (impl)
macro_rules! Depcrate_tz_offsetimpl_1000 {
() => {
// Module: crate::tz::offset
// Provides: {"impl_1000"}
// Dependencies: {}
# [doc = " Subtracts a signed duration of time from an offset in place. This panics on"] # [doc = " overflow."] # [doc = ""] # [doc = " For checked arithmetic, see [`Offset::checked_sub`]."] impl SubAssign < SignedDuration > for Offset { # [inline] fn sub_assign (& mut self , rhs : SignedDuration) { * self = self . sub (rhs) ; } }
};
}
