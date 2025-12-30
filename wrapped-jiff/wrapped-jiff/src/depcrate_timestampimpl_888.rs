// Generated macro for impl_888 (impl)
macro_rules! Depcrate_timestampimpl_888 {
() => {
// Module: crate::timestamp
// Provides: {"impl_888"}
// Dependencies: {}
# [doc = " Subtracts an unsigned duration of time from a timestamp in place."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Timestamp::checked_sub`]."] impl core :: ops :: SubAssign < UnsignedDuration > for Timestamp { # [inline] fn sub_assign (& mut self , rhs : UnsignedDuration) { * self = * self - rhs } }
};
}
