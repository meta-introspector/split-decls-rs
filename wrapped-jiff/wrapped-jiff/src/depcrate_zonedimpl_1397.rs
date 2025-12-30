// Generated macro for impl_1397 (impl)
macro_rules! Depcrate_zonedimpl_1397 {
() => {
// Module: crate::zoned
// Provides: {"impl_1397"}
// Dependencies: {}
# [doc = " Adds a signed duration of time to a zoned datetime in place."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Zoned::checked_add`]."] impl core :: ops :: AddAssign < SignedDuration > for Zoned { # [inline] fn add_assign (& mut self , rhs : SignedDuration) { * self = & * self + rhs } }
};
}
