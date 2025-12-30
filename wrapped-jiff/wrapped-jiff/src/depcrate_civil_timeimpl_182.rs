// Generated macro for impl_182 (impl)
macro_rules! Depcrate_civil_timeimpl_182 {
() => {
// Module: crate::civil::time
// Provides: {"impl_182"}
// Dependencies: {}
# [doc = " Adds a signed duration of time in place. This uses wrapping arithmetic."] # [doc = ""] # [doc = " For checked arithmetic, see [`Time::checked_add`]."] impl core :: ops :: AddAssign < SignedDuration > for Time { # [inline] fn add_assign (& mut self , rhs : SignedDuration) { * self = * self + rhs ; } }
};
}
