// Generated macro for impl_106 (impl)
macro_rules! Depcrate_civil_datetimeimpl_106 {
() => {
// Module: crate::civil::datetime
// Provides: {"impl_106"}
// Dependencies: {}
# [doc = " Subtracts a signed duration of time from a datetime in place."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`DateTime::checked_sub`]."] impl core :: ops :: SubAssign < SignedDuration > for DateTime { # [inline] fn sub_assign (& mut self , rhs : SignedDuration) { * self = * self - rhs } }
};
}
