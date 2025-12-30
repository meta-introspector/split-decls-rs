// Generated macro for impl_41 (impl)
macro_rules! Depcrate_civil_dateimpl_41 {
() => {
// Module: crate::civil::date
// Provides: {"impl_41"}
// Dependencies: {}
# [doc = " Subtracts a span of time from a date in place."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Date::checked_sub`]."] impl core :: ops :: SubAssign < Span > for Date { # [inline] fn sub_assign (& mut self , rhs : Span) { * self = * self - rhs ; } }
};
}
