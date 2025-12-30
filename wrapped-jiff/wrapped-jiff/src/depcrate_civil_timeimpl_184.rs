// Generated macro for impl_184 (impl)
macro_rules! Depcrate_civil_timeimpl_184 {
() => {
// Module: crate::civil::time
// Provides: {"impl_184"}
// Dependencies: {}
# [doc = " Subtracts a signed duration of time in place. This uses wrapping arithmetic."] # [doc = ""] # [doc = " For checked arithmetic, see [`Time::checked_sub`]."] impl core :: ops :: SubAssign < SignedDuration > for Time { # [inline] fn sub_assign (& mut self , rhs : SignedDuration) { * self = * self - rhs ; } }
};
}
