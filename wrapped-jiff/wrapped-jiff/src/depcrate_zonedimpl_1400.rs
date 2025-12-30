// Generated macro for impl_1400 (impl)
macro_rules! Depcrate_zonedimpl_1400 {
() => {
// Module: crate::zoned
// Provides: {"impl_1400"}
// Dependencies: {}
# [doc = " Subtracts a signed duration of time from a zoned datetime in place."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Zoned::checked_sub`]."] impl core :: ops :: SubAssign < SignedDuration > for Zoned { # [inline] fn sub_assign (& mut self , rhs : SignedDuration) { * self = & * self - rhs } }
};
}
