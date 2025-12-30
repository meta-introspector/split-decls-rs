// Generated macro for impl_44 (impl)
macro_rules! Depcrate_civil_dateimpl_44 {
() => {
// Module: crate::civil::date
// Provides: {"impl_44"}
// Dependencies: {}
# [doc = " Adds a signed duration of time to a date in place."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Date::checked_add`]."] impl core :: ops :: AddAssign < SignedDuration > for Date { # [inline] fn add_assign (& mut self , rhs : SignedDuration) { * self = * self + rhs ; } }
};
}
