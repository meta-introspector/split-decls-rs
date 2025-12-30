// Generated macro for impl_1002 (impl)
macro_rules! Depcrate_tz_offsetimpl_1002 {
() => {
// Module: crate::tz::offset
// Provides: {"impl_1002"}
// Dependencies: {}
# [doc = " Adds an unsigned duration of time to an offset in place. This panics on"] # [doc = " overflow."] # [doc = ""] # [doc = " For checked arithmetic, see [`Offset::checked_add`]."] impl AddAssign < UnsignedDuration > for Offset { # [inline] fn add_assign (& mut self , rhs : UnsignedDuration) { * self = self . add (rhs) ; } }
};
}
