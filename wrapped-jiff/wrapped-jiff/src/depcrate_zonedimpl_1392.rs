// Generated macro for impl_1392 (impl)
macro_rules! Depcrate_zonedimpl_1392 {
() => {
// Module: crate::zoned
// Provides: {"impl_1392"}
// Dependencies: {}
# [doc = " Subtracts a span of time from a zoned datetime in place."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Zoned::checked_sub`]."] impl core :: ops :: SubAssign < Span > for Zoned { # [inline] fn sub_assign (& mut self , rhs : Span) { * self = & * self - rhs } }
};
}
