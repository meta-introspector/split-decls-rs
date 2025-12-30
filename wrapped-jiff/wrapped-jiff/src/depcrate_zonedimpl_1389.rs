// Generated macro for impl_1389 (impl)
macro_rules! Depcrate_zonedimpl_1389 {
() => {
// Module: crate::zoned
// Provides: {"impl_1389"}
// Dependencies: {}
# [doc = " Adds a span of time to a zoned datetime in place."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Zoned::checked_add`]."] impl core :: ops :: AddAssign < Span > for Zoned { # [inline] fn add_assign (& mut self , rhs : Span) { * self = & * self + rhs } }
};
}
