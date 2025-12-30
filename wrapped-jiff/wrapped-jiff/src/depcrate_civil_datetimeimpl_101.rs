// Generated macro for impl_101 (impl)
macro_rules! Depcrate_civil_datetimeimpl_101 {
() => {
// Module: crate::civil::datetime
// Provides: {"impl_101"}
// Dependencies: {}
# [doc = " Subtracts a span of time from a datetime in place."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`DateTime::checked_sub`]."] impl core :: ops :: SubAssign < Span > for DateTime { # [inline] fn sub_assign (& mut self , rhs : Span) { * self = * self - rhs } }
};
}
