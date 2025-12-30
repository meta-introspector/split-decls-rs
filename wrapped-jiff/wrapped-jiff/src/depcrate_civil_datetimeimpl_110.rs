// Generated macro for impl_110 (impl)
macro_rules! Depcrate_civil_datetimeimpl_110 {
() => {
// Module: crate::civil::datetime
// Provides: {"impl_110"}
// Dependencies: {}
# [doc = " Subtracts an unsigned duration of time from a datetime in place."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`DateTime::checked_sub`]."] impl core :: ops :: SubAssign < UnsignedDuration > for DateTime { # [inline] fn sub_assign (& mut self , rhs : UnsignedDuration) { * self = * self - rhs } }
};
}
