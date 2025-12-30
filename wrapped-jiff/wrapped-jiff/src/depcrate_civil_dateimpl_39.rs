// Generated macro for impl_39 (impl)
macro_rules! Depcrate_civil_dateimpl_39 {
() => {
// Module: crate::civil::date
// Provides: {"impl_39"}
// Dependencies: {}
# [doc = " Adds a span of time to a date in place."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Date::checked_add`]."] impl core :: ops :: AddAssign < Span > for Date { # [inline] fn add_assign (& mut self , rhs : Span) { * self = * self + rhs ; } }
};
}
