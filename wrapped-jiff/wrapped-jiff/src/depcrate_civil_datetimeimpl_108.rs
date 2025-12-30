// Generated macro for impl_108 (impl)
macro_rules! Depcrate_civil_datetimeimpl_108 {
() => {
// Module: crate::civil::datetime
// Provides: {"impl_108"}
// Dependencies: {}
# [doc = " Adds an unsigned duration of time to a datetime in place."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`DateTime::checked_add`]."] impl core :: ops :: AddAssign < UnsignedDuration > for DateTime { # [inline] fn add_assign (& mut self , rhs : UnsignedDuration) { * self = * self + rhs } }
};
}
