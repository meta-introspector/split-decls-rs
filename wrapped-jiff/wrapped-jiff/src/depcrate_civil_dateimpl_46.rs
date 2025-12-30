// Generated macro for impl_46 (impl)
macro_rules! Depcrate_civil_dateimpl_46 {
() => {
// Module: crate::civil::date
// Provides: {"impl_46"}
// Dependencies: {}
# [doc = " Subtracts a signed duration of time from a date in place."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Date::checked_sub`]."] impl core :: ops :: SubAssign < SignedDuration > for Date { # [inline] fn sub_assign (& mut self , rhs : SignedDuration) { * self = * self - rhs ; } }
};
}
