// Generated macro for impl_50 (impl)
macro_rules! Depcrate_civil_dateimpl_50 {
() => {
// Module: crate::civil::date
// Provides: {"impl_50"}
// Dependencies: {}
# [doc = " Subtracts an unsigned duration of time from a date in place."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Date::checked_sub`]."] impl core :: ops :: SubAssign < UnsignedDuration > for Date { # [inline] fn sub_assign (& mut self , rhs : UnsignedDuration) { * self = * self - rhs ; } }
};
}
