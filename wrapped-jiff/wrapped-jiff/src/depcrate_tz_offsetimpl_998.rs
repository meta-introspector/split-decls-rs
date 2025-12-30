// Generated macro for impl_998 (impl)
macro_rules! Depcrate_tz_offsetimpl_998 {
() => {
// Module: crate::tz::offset
// Provides: {"impl_998"}
// Dependencies: {}
# [doc = " Adds a signed duration of time to an offset in place. This panics on"] # [doc = " overflow."] # [doc = ""] # [doc = " For checked arithmetic, see [`Offset::checked_add`]."] impl AddAssign < SignedDuration > for Offset { # [inline] fn add_assign (& mut self , rhs : SignedDuration) { * self = self . add (rhs) ; } }
};
}
