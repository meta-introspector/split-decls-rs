// Generated macro for impl_886 (impl)
macro_rules! Depcrate_timestampimpl_886 {
() => {
// Module: crate::timestamp
// Provides: {"impl_886"}
// Dependencies: {}
# [doc = " Adds an unsigned duration of time to a timestamp in place."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Timestamp::checked_add`]."] impl core :: ops :: AddAssign < UnsignedDuration > for Timestamp { # [inline] fn add_assign (& mut self , rhs : UnsignedDuration) { * self = * self + rhs } }
};
}
