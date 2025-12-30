// Generated macro for impl_48 (impl)
macro_rules! Depcrate_civil_dateimpl_48 {
() => {
// Module: crate::civil::date
// Provides: {"impl_48"}
// Dependencies: {}
# [doc = " Adds an unsigned duration of time to a date in place."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Date::checked_add`]."] impl core :: ops :: AddAssign < UnsignedDuration > for Date { # [inline] fn add_assign (& mut self , rhs : UnsignedDuration) { * self = * self + rhs ; } }
};
}
