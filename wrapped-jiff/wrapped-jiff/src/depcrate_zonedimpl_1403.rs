// Generated macro for impl_1403 (impl)
macro_rules! Depcrate_zonedimpl_1403 {
() => {
// Module: crate::zoned
// Provides: {"impl_1403"}
// Dependencies: {}
# [doc = " Adds an unsigned duration of time to a zoned datetime in place."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Zoned::checked_add`]."] impl core :: ops :: AddAssign < UnsignedDuration > for Zoned { # [inline] fn add_assign (& mut self , rhs : UnsignedDuration) { * self = & * self + rhs } }
};
}
