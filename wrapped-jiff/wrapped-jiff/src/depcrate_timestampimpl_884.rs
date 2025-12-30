// Generated macro for impl_884 (impl)
macro_rules! Depcrate_timestampimpl_884 {
() => {
// Module: crate::timestamp
// Provides: {"impl_884"}
// Dependencies: {}
# [doc = " Subtracts a signed duration of time from a timestamp in place."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Timestamp::checked_sub`]."] impl core :: ops :: SubAssign < SignedDuration > for Timestamp { # [inline] fn sub_assign (& mut self , rhs : SignedDuration) { * self = * self - rhs } }
};
}
