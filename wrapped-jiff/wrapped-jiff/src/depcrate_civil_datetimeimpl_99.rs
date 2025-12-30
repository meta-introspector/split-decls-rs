// Generated macro for impl_99 (impl)
macro_rules! Depcrate_civil_datetimeimpl_99 {
() => {
// Module: crate::civil::datetime
// Provides: {"impl_99"}
// Dependencies: {}
# [doc = " Adds a span of time to a datetime in place."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`DateTime::checked_add`]."] impl core :: ops :: AddAssign < Span > for DateTime { # [inline] fn add_assign (& mut self , rhs : Span) { * self = * self + rhs } }
};
}
