// Generated macro for impl_188 (impl)
macro_rules! Depcrate_civil_timeimpl_188 {
() => {
// Module: crate::civil::time
// Provides: {"impl_188"}
// Dependencies: {}
# [doc = " Subtracts an unsigned duration of time in place. This uses wrapping"] # [doc = " arithmetic."] # [doc = ""] # [doc = " For checked arithmetic, see [`Time::checked_sub`]."] impl core :: ops :: SubAssign < UnsignedDuration > for Time { # [inline] fn sub_assign (& mut self , rhs : UnsignedDuration) { * self = * self - rhs ; } }
};
}
