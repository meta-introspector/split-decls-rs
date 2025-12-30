// Generated macro for impl_882 (impl)
macro_rules! Depcrate_timestampimpl_882 {
() => {
// Module: crate::timestamp
// Provides: {"impl_882"}
// Dependencies: {}
# [doc = " Adds a signed duration of time to a timestamp in place."] # [doc = ""] # [doc = " This uses checked arithmetic and panics on overflow. To handle overflow"] # [doc = " without panics, use [`Timestamp::checked_add`]."] impl core :: ops :: AddAssign < SignedDuration > for Timestamp { # [inline] fn add_assign (& mut self , rhs : SignedDuration) { * self = * self + rhs } }
};
}
