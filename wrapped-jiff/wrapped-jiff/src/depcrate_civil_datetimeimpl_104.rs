// Generated macro for impl_104 (impl)
macro_rules! Depcrate_civil_datetimeimpl_104 {
() => {
// Module: crate::civil::datetime
// Provides: {"impl_104"}
// Dependencies: {}
# [doc = " Adds a signed duration of time to a datetime in place."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`DateTime::checked_add`]."] impl core :: ops :: AddAssign < SignedDuration > for DateTime { # [inline] fn add_assign (& mut self , rhs : SignedDuration) { * self = * self + rhs } }
};
}
