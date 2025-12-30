// Generated macro for impl_186 (impl)
macro_rules! Depcrate_civil_timeimpl_186 {
() => {
// Module: crate::civil::time
// Provides: {"impl_186"}
// Dependencies: {}
# [doc = " Adds an unsigned duration of time in place. This uses wrapping arithmetic."] # [doc = ""] # [doc = " For checked arithmetic, see [`Time::checked_add`]."] impl core :: ops :: AddAssign < UnsignedDuration > for Time { # [inline] fn add_assign (& mut self , rhs : UnsignedDuration) { * self = * self + rhs ; } }
};
}
