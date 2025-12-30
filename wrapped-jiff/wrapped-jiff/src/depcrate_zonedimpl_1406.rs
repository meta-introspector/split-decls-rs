// Generated macro for impl_1406 (impl)
macro_rules! Depcrate_zonedimpl_1406 {
() => {
// Module: crate::zoned
// Provides: {"impl_1406"}
// Dependencies: {}
# [doc = " Subtracts an unsigned duration of time from a zoned datetime in place."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Zoned::checked_sub`]."] impl core :: ops :: SubAssign < UnsignedDuration > for Zoned { # [inline] fn sub_assign (& mut self , rhs : UnsignedDuration) { * self = & * self - rhs } }
};
}
